use hex;
use pbkdf2::pbkdf2_hmac;
use rpassword::read_password;
use sha2::Sha256;
use std::io::{self, Write};

const SALT: &'static [u8] = b"useless_salt";

/// Reads user input from stdin
pub fn read_from_stdin() -> io::Result<String> {
    std::io::stdout().flush().unwrap();
    read_password()
}

/// Hash given password string
pub fn hash_password_string(password: String) -> String {
    let mut res = [0u8; 20];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), SALT, 1000, &mut res);
    hex::encode(&res)
}

/// Verify that given password matches with given hash
pub fn verify_password_hash(password: String, hash: String) -> bool {
    hash == hash_password_string(password)
}
