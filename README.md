Rust installation
If you did not set up Rust during lecture, you’ll need to install it first.

Open a terminal and enter the following command:

$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
The command downloads and runs the rustup script, which installs Rust. :

Rust is installed now. Great!
I highly recommend installing VSCode and its Rust extension before you start this assignment.

Starter code
The starter code can be found in the GitHub repository, under assignment-2. If you have not cloned the repository to your machine, do so now:

$ git clone https://github.com/spacelab-ccny/sse-sp24.git
If you already have it, you can perform an update:

$ cd sse-sp24/
$ git stash --include-untracked
$ git pull
$ git stash pop
These commands temporarily stash any unsaved changes, pull the new code, and restore your unsaved changes from the stash.

Functions to implement
The following are the functions you have to implement for this assignment. The functionality for each is described below. It is up to you to implement and test this functionality using Rust. You may only modify the body of these functions. You may not modify the function signatures, i.e., the arguments and return values of each function must stay the same.

save_to_file_as_b64
fn save_to_file_as_b64(file_name: &str, data: &[u8])
Encode data as Base64 using the engine BASE64_STANDARD.
Write the contents of the Base64 string to the file given by file_name.
read_from_b64_file
fn read_from_b64_file(file_name: &str) -> Vec<u8>
Read the contents of the file given by file_name.
Decode the contents of the file using the engine BASE64_STANDARD.
Return the decoded bytes.
keygen

fn keygen() -> ([u8; 32], [u8; 32])
Generate a StaticSecret from random.
Generate a PublicKey from this StaticSecret.
Convert the secret and public keys to bytes.
Return a tuple of the secret key bytes and public key bytes.
encrypt


fn encrypt(input: Vec<u8>, sender_sk: [u8; 32], receiver_pk: [u8; 32]) -> Vec<u8>
Convert the sender secret key array into a StaticSecret.
Convert the receiver public key array into a PublicKey.
Perform Diffie-Hellman key exchange to generate a SharedSecret.
Hash the SharedSecret into 32 bytes using SHA-256.
Transform the hashed bytes into an AES-256-GCM key (Key<Aes256Gcm>).
Generate a random nonce for AES-256-GCM.
Encrypt the input under the AES-256-GCM key and nonce.
Append the nonce to the end of the output vector containing the ciphertext.
Return the vector of bytes containing the ciphertext and the nonce.


decrypt
fn decrypt(input: Vec<u8>, receiver_sk: [u8; 32], sender_pk: [u8; 32]) -> Vec<u8>
Convert the receiver secret key array into a StaticSecret.
Convert the sender public key array into a PublicKey.
Perform Diffie-Hellman key exchange to generate a SharedSecret.
Hash the SharedSecret into 32 bytes using SHA-256.
Transform the hashed bytes into an AES-256-GCM key (Key<Aes256Gcm>).
Extract the ciphertext and the nonce from input. The last 12 bytes of input contains the nonce (which we generated in Step 6 of encrypt), while the rest of input contains the ciphertext to decrypt (which we generated in Step 7 of encrypt).
Decrypt the ciphertext using the AES-256-GCM key and nonce.
Return the vector of bytes containing the plaintext data.
Command-line interface

The main() function in the code handles command parsing and calling functions. You may not modify this function. The different commands supported by the command-line interface are below. Take a look through the code to see how main() handles each command.

cargo run keygen <secret_key> <public_key>
Generate a keypair for encryption.

<secret_key>: filename to store the generated secret key
<public_key>: filename to store the public key associated with the generated secret key
cargo run encrypt <input> <output> <sender_secret_key> <receiever_public_key>
Encrypt a file to a receiver.

<input>: filename of the file to encrypt
<output>: filename to store the encrypted file
<sender_secret_key>: filename of the secret key of the sender (generated by keygen)
<receiver_secret_key>: filename of the public key of the receiver (generated by keygen)
cargo run decrypt <input> <output> <receiver_secret_key> <sender_public_key>
Decrypt a file from a sender.

<input>: filename of the file to decrypt
<output>: filename to store the decrypted file
<receiver_secret_key>: filename of the secret key of the receiver (generated by keygen)
<sender_secret_key>: filename of the public key of the sender (generated by keygen)
Dependencies and crates
To implement the above functions, you will need to use external crates. You are allowed to only use the crates that are listed in Cargo.toml. You may not add new crates as dependencies. Here are links to the documentation pages for each of the crates used:

x25519-dalek for Diffie-Hellman key exchange
sha2 for SHA-256 hashing
aes_gcm for AES-256-GCM authenticated encryption
base64 for Base64 encoding and decoding
The documentation for all of these crates is quite good, and will be critical as you implement the functions in this code.

Tests
You will need to implement some tests to make sure your code works. It is up to you how you decide to write tests, and for what portions of your code. This may require you splitting up functions so you can test them easier. Part of your grade will be determined by your approach to testing. You may consider applying test-driven development to write your code, but this is not a hard requirement.

Tips
Start early. Rust is a new language and cryptographic engineering is a new concept (for most of you, at least). This is not an assignment you can complete a day or two before the deadline.
Read the documentation pages for the crates first. The documentation for these crates will be key in helping you implement the required functionality. If you have a question, start there first.
The dbg!() macro can help with printing out debug values of your code. Because we’re dealing with bytes mostly, it can be helpful to convert the bytes to Base64, and then call dbg!() on the result to print them out.
Robustness is not being graded on this assignment. Usually, you would have to gracefully handle errors, but for this assignment, you can panic! on an error. This means you are free to use unwrap() to bypass error handling.
Help each other out. Work with your partner (perhaps by using pair programming) and contribute on the Blackboard discussion board with your questions. Emailing the instructor is a last resort.
Examples
There are two examples located in the examples/ directory with files generated using the solution implementation. Use these files to help develop your solution. Be careful – if you overwrite the files, you may find that your values are different. Make sure you’re using the files (from the repository) when testing against these examples.

cd examples/
There are two parties, A and B.

cargo run keygen a_sk.txt a_pk.txt    # party A
cargo run keygen b_sk.txt b_pk.txt    # party B
The Base64 in a_sk.txt corresponds to the following bytes:

[77, 105, 123, 62, 170, 198, 29, 150, 82, 70, 152, 150, 38, 114, 94, 160, 7, 84, 131, 221, 130, 89, 77, 243, 191, 147, 174, 121, 49, 91, 187, 214]
The Base64 in a_pk.txt corresponds to the following bytes:

[30, 142, 43, 24, 172, 129, 55, 138, 115, 90, 233, 202, 162, 74, 49, 37, 111, 215, 214, 13, 51, 75, 19, 255, 87, 44, 170, 227, 217, 121, 217, 34]
The Base64 in b_sk.txt corresponds to the following bytes:

[45, 203, 5, 168, 176, 17, 244, 93, 85, 7, 38, 91, 166, 223, 208, 58, 83, 180, 175, 225, 226, 207, 80, 104, 97, 11, 46, 234, 214, 48, 39, 37]
The Base64 in b_pk.txt corresponds to the following bytes:

[246, 88, 196, 62, 121, 69, 20, 123, 199, 128, 26, 114, 238, 35, 255, 153, 209, 43, 110, 231, 78, 227, 115, 192, 90, 20, 40, 5, 151, 98, 253, 123]
Of course, Diffie-Hellman keys are randomly generated, so your implementation will generate different keys from the ones in these files. But, you should be able to use them to check the format, and to test your other functions (like the Base64 ones).

Example 1
Example 2 was generated with A as the sender, and B as the receiver.

cargo run encrypt example1.jpg example1_enc.txt a_sk.txt b_pk.txt 
cargo run decrypt example1_enc.txt example1_out.jpg b_sk.txt a_pk.txt
The Base64-encoded version of the shared secret after Diffie-Hellman key exchange:

Ls8Cio9YikjyTprhrIGBtWrIHOeYb4NImLmn0WTk+Sg=
The Base64-encoded version of the key used to encrypt/decrypt (i.e., the SHA-256 hash of the shared secret):

39mVQjDXJsq4qErg27GbbagBWGpRwtmAEGz2GVo0Chc=
Note that, because nonces are randomly generated, your encryption function’s example1_enc.txt will differ from the one in the repository. But, the shared secret will be the same, and your decryption function should be able to decrypt the example from the repository into example1_out.jpg.

Example 2
Example 2 was generated with B as the sender, and A as the receiver.

cargo run encrypt example2.jpg example2_enc.txt b_sk.txt a_pk.txt  
cargo run decrypt example2_enc.txt example2_out.jpg a_sk.txt b_pk.txt
The Base64-encoded version of the shared secret after Diffie-Hellman key exchange:

Ls8Cio9YikjyTprhrIGBtWrIHOeYb4NImLmn0WTk+Sg=
The Base64-encoded version of the key used to encrypt/decrypt (i.e., the SHA-256 hash of the shared secret):

39mVQjDXJsq4qErg27GbbagBWGpRwtmAEGz2GVo0Chc=
These are indeed the same as in Example 1, which makes sense: regardless of who initiates the key exchange (A or B), the shared secret is the same.

Again, because nonces are randomly generated, your encryption function’s example2_enc.txt will differ from the one in the repository. But, the shared secret will be the same, and your decryption function should be able to decrypt the example from the repository into example2_out.jpg.
