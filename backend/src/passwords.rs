use bcrypt::{hash, verify, DEFAULT_COST};

pub fn generate_hash (plainpass: String) -> String {
    let hashed = hash(plainpass, DEFAULT_COST).unwrap();
    hashed
}

pub fn verify_hash (plainpass: String, hashed: String) -> bool {
    let result = verify(plainpass, &hashed).unwrap();
    result
}