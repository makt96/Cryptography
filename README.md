# Rust Installation Guide

## Getting Started

### Installing Rust
If you did not set up Rust , you'll need to install it first. Open a terminal and enter the following command:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh


The starter code can be found in the GitHub repository, under assignment-2. If you have not cloned the repository to your machine, do so now:

git clone https://github.com/makt96/Cryptography.git

cd Cryptography/
git stash --include-untracked
git pull
git stash pop

```
### Dependencies and crates


Use only the crates listed in Cargo.toml. Here are the primary crates required:

x25519-dalek for Diffie-Hellman key exchange.
sha2 for SHA-256 hashing.
aes_gcm for AES-256-GCM authenticated encryption.
base64 for Base64 encoding and decoding.
Refer to the documentation of these crates as needed.






# Examples

To facilitate the development and testing of your solution, two examples are provided in the `examples/` directory. Ensure caution when working with these files, as overwriting them may result in different values. Use the provided files from the repository for accurate testing against these examples.

Navigate to the examples directory:


cd examples/
Key Generation
Two parties, A and B, each generate a key pair using the keygen command:

Party A

```bash
cargo run keygen a_sk.txt a_pk.txt
The Base64 content in a_sk.txt corresponds to the following bytes:

[77, 105, 123, 62, 170, 198, 29, 150, 82, 70, 152, 150, 38, 114, 94, 160, 7, 84, 131, 221, 130, 89, 77, 243, 191, 147, 174, 121, 49, 91, 187, 214]

The Base64 content in a_pk.txt corresponds to the following bytes:

[30, 142, 43, 24, 172, 129, 37, 111, 215, 214, 13, 51, 75, 19, 255, 87, 44, 170, 227, 217, 121, 217, 34]

```
Party B

```bash
cargo run keygen b_sk.txt b_pk.txt

The Base64 content in b_sk.txt corresponds to the following bytes:

[45, 203, 5, 168, 176, 17, 244, 93, 85, 7, 38, 91, 166, 223, 208, 58, 83, 180, 175, 225, 226, 207, 80, 104, 97, 11, 46, 234, 214, 48, 39, 37]

The Base64 content in b_pk.txt corresponds to the following bytes:

[246, 88, 196, 62, 121, 69, 20, 123, 199, 128, 26, 114, 238, 35, 255, 153, 209, 43, 110, 231, 78, 227, 115, 192, 90, 20, 40, 5, 151, 98, 253, 123]

Remember, Diffie-Hellman keys are randomly generated, so your implementation will generate different keys.
```
########## Example 1

Example 1 was generated with A as the sender and B as the receiver. Encrypt and decrypt the file using the following commands:

```bash
cargo run encrypt example1.jpg example1_enc.txt a_sk.txt b_pk.txt
cargo run decrypt example1_enc.txt example1_out.jpg b_sk.txt a_pk.txt

```

The Base64-encoded version of the shared secret after Diffie-Hellman key exchange:

Ls8Cio9YikjyTprhrIGBtWrIHOeYb4NImLmn0WTk+Sg=

The Base64-encoded version of the key used for encryption/decryption (SHA-256 hash of the shared secret):

39mVQjDXJsq4qErg27GbbagBWGpRwtmAEGz2GVo0Chc=

Note that due to randomly generated nonces, your encryption function's example1_enc.txt may differ from the repository's, but the shared secret remains the same.

########## Example 2
Example 2 was generated with B as the sender and A as the receiver. Encrypt and decrypt the file using the following commands:

```bash
cargo run encrypt example2.jpg example2_enc.txt b_sk.txt a_pk.txt
cargo run decrypt example2_enc.txt example2_out.jpg a_sk.txt b_pk.txt

```
The Base64-encoded version of the shared secret after Diffie-Hellman key exchange:

Ls8Cio9YikjyTprhrIGBtWrIHOeYb4NImLmn0WTk+Sg=

The Base64-encoded version of the key used for encryption/decryption (SHA-256 hash of the shared secret):

39mVQjDXJsq4qErg27GbbagBWGpRwtmAEGz2GVo0Chc=

Similar to Example 1, your encryption function's example2_enc.txt may differ, but the shared secret remains constant.

Remember to use these examples as a reference to validate your implementation.
