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
