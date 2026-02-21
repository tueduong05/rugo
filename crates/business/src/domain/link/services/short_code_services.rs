pub trait ShortCodeGenerator: Send + Sync {
    fn generate(&self) -> String;
}
