use business::domain::link::services::short_code_services::ShortCodeGenerator;

pub struct MockShortCodeGenerator;

impl ShortCodeGenerator for MockShortCodeGenerator {
    fn generate(&self) -> String {
        String::from("ShortCode")
    }
}
