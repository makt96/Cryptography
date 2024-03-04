use std::fs;
use std::fs::File;
use aes_gcm::{
    aead::{Aead, OsRng},
     Aes256Gcm, 
     Key, KeyInit,
     Nonce
};
use base64::prelude::*;
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey, StaticSecret};
use std::io::{Read, Write};



fn save_to_file_as_b64(file_name: &str, data: &[u8]) {
       // Encode the data to Base64 using the standard Base64 alphabet.
       let encoded_data = BASE64_STANDARD.encode(data);

       // Open a file in write mode, or create it if it doesn't exist.
       let mut file = match File::create(file_name) {
           Ok(file) => file,
           Err(err) => {
               eprintln!("Error creating file: {}", err);
               return;
           }
       };
   
       // Write the Base64 encoded data to the file.
       if let Err(err) = file.write_all(encoded_data.as_bytes()) {
           eprintln!("Error writing to file: {}", err);
       } else {
           println!("Data saved to file successfully.");
       }
    
}


fn read_from_b64_file(file_name: &str) -> Vec<u8> {
     // Open the file in read mode.
     let file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return Vec::new(); // Return an empty Vec<u8> on error
        }
    };

    // Read the contents of the file into a Vec<u8>.
    let mut encoded_data = Vec::new();
    if let Err(err) = file.take(u64::MAX as u64).read_to_end(&mut encoded_data) {
        eprintln!("Error reading from file: {}", err);
        return Vec::new(); // Return an empty Vec<u8> on error
    }

    // Decode the Base64-encoded data.
    match BASE64_STANDARD.decode(&encoded_data) {
        Ok(decoded) => decoded,
        Err(err) => {
            eprintln!("Error decoding Base64: {}", err);
            Vec::new() // Return an empty Vec<u8> on error
        }
    }
}


fn keygen() -> ([u8; 32], [u8; 32]) {
    // Generate a random StaticSecret
    let secret_key = StaticSecret::random_from_rng(&mut OsRng);

    // Derive the corresponding PublicKey
    let public_key: PublicKey = (&secret_key).into();

    // Convert the secret and public keys to bytes
    let secret_key_bytes = secret_key.to_bytes();
    let public_key_bytes = public_key.as_bytes().to_owned();

    (secret_key_bytes, public_key_bytes)
}

fn encrypt(input: Vec<u8>, sender_sk: [u8; 32], receiver_pk: [u8; 32]) -> Vec<u8> {
    // Convert sender secret key array into a StaticSecret
    let _sender_secret_key = StaticSecret::from(sender_sk);

    // Convert receiver public key array into a PublicKey
    let receiver_public_key = PublicKey::from(receiver_pk);

    let shared_secret = _sender_secret_key.diffie_hellman(&receiver_public_key);

    // Hash the SharedSecret into 32 bytes using SHA-256
    let hashed_secret = Sha256::digest(&shared_secret);

    // Transform the hashed bytes into an AES-256-GCM key
    let aes_key = Key::<Aes256Gcm>::from_slice(&hashed_secret);

    // Generate a random nonce for AES-256-GCM
    let nonce = Nonce::from_slice(&hashed_secret[0..12]); // Use the first 12 bytes as the nonce

    // Encrypt the input under the AES-256-GCM key and nonce
    let cipher = Aes256Gcm::new(aes_key);
    let ciphertext = cipher.encrypt(nonce, input.as_ref()).expect("Encryption failed");

    // Append the nonce to the end of the output vector containing the ciphertext
    let mut output = Vec::new();
    output.extend_from_slice(&ciphertext);
    output.extend_from_slice(nonce.as_slice());

    output
}


fn decrypt(input: Vec<u8>, receiver_sk: [u8; 32], sender_pk: [u8; 32]) -> Vec<u8> {
   // Convert the receiver secret key array into a StaticSecret
   let receiver_secret_key = StaticSecret::from(receiver_sk);

   // Convert the sender public key array into a PublicKey
   let sender_public_key = PublicKey::from(sender_pk);

   // Perform Diffie-Hellman key exchange to generate a SharedSecret
   let shared_secret = receiver_secret_key.diffie_hellman(&sender_public_key);

   // Hash the SharedSecret into 32 bytes using SHA-256
   let hashed_secret = Sha256::digest(shared_secret.as_bytes());

   // Transform the hashed bytes into an AES-256-GCM key (Key<Aes256Gcm>)
   let aes_key = Key::<Aes256Gcm>::from_slice(&hashed_secret);

   // Extract the ciphertext and the nonce from input
   let nonce_bytes = &input[input.len() - 12..];
   let ciphertext = &input[..input.len() - 12];

   // Decrypt the ciphertext using the AES-256-GCM key and nonce
   let nonce = Nonce::from_slice(nonce_bytes);
   let cipher = Aes256Gcm::new(aes_key);

   let plaintext = cipher.decrypt(nonce, ciphertext).expect("decryption failure!");

   plaintext.to_vec()
}


fn main() {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Command parsing: keygen, encrypt, decrypt
    let cmd = &args[1];
    if cmd == "keygen" {
        // Arguments to the command
        let secret_key = &args[2];
        let public_key = &args[3];

        // Generate a secret and public key for this user
        let (sk_bytes, pk_bytes) = keygen();

        // Save those bytes as Base64 to file
        save_to_file_as_b64(&secret_key, &sk_bytes);
        save_to_file_as_b64(&public_key, &pk_bytes);
    } else if cmd == "encrypt" {
        // Arguments to the command
        let input = &args[2];
        let output = &args[3];
        let sender_sk = &args[4];
        let receiver_pk = &args[5];

        // Read input from file
        // Note that this input is not necessarily Base64-encoded
        let input = fs::read(input).unwrap();

        // Read the base64-encoded secret and public keys from file
        // Need to convert the Vec<u8> from this function into the 32-byte array for each key
        let sender_sk: [u8; 32] = read_from_b64_file(sender_sk).try_into().unwrap();
        let receiver_pk: [u8; 32] = read_from_b64_file(receiver_pk).try_into().unwrap();

        // Call the encryption operation
        let output_bytes = encrypt(input, sender_sk, receiver_pk);

        // Save those bytes as Base64 to file
        save_to_file_as_b64(&output, &output_bytes);
    } else if cmd == "decrypt" {
        // Arguments to the command
        let input = &args[2];
        let output = &args[3];
        let receiver_sk = &args[4];
        let sender_pk = &args[5];

        // Read the Base64-encoded input ciphertext from file
        let input = read_from_b64_file(&input);

        // Read the base64-encoded secret and public keys from file
        // Need to convert the Vec<u8> from this function into the 32-byte array for each key
        let receiver_sk: [u8; 32] = read_from_b64_file(&receiver_sk).try_into().unwrap();
        let sender_pk: [u8; 32] = read_from_b64_file(&sender_pk).try_into().unwrap();

        // Call the decryption operation
        let output_bytes = decrypt(input, receiver_sk, sender_pk);

        // Save those bytes as Base64 to file
        fs::write(output, output_bytes).unwrap();
    } else {
        panic!("command not found!")
    }
}

#[cfg(test)]
mod tests {
        use super::*;
    
        #[test]
        fn test_keygen() {
            let (sk_bytes, pk_bytes) = keygen();
    
            assert_eq!(sk_bytes.len(), 32);
            assert_eq!(pk_bytes.len(), 32);
        }
    
}
