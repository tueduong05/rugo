use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher as ArgonHasher, PasswordVerifier, SaltString},
};
use business::domain::common::{
    services::password_services::{PasswordHasher, PasswordPolicy},
    value_objects::hashed_password::HashedPassword,
};
use password_hash::rand_core::OsRng;
use zxcvbn::{Score, zxcvbn};

pub struct ZxcvbnPolicy {
    pub min_score: Score,
}

impl ZxcvbnPolicy {
    pub fn new(score: u8) -> Self {
        let min_score = match score {
            0 => Score::Zero,
            1 => Score::One,
            2 => Score::Two,
            3 => Score::Three,
            _ => Score::Four,
        };

        Self { min_score }
    }
}

impl PasswordPolicy for ZxcvbnPolicy {
    fn validate(&self, password: &str) -> bool {
        let estimate = zxcvbn(password, &[]);
        estimate.score() >= self.min_score
    }
}

pub struct Argon2idHasher;

impl PasswordHasher for Argon2idHasher {
    fn hash(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Could not hash password")
            .to_string()
    }

    fn verify(&self, password: &str, hash: &HashedPassword) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(hash.as_str()) else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
