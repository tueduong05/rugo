use business::domain::link::services::short_code_services::ShortCodeGenerator;
use rand::{Rng, distr::Alphanumeric};

pub struct RandomShortCodeGenerator;

impl ShortCodeGenerator for RandomShortCodeGenerator {
    fn generate(&self) -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect()
    }
}
