use business::domain::link_analytics::{
    services::UserAgentParser, value_objects::user_agent::UserAgent,
};
use woothee::parser::Parser;

#[derive(Default)]
pub struct WootheeUserAgentParser {
    parser: Parser,
}

impl WootheeUserAgentParser {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }
}

impl UserAgentParser for WootheeUserAgentParser {
    fn parse(&self, ua_string: &Option<String>) -> UserAgent {
        let result = ua_string.as_ref().and_then(|ua| self.parser.parse(ua));

        match result {
            Some(data) => UserAgent::new(
                data.name.to_string(),
                data.os.to_string(),
                data.category.to_string(),
            ),

            None => UserAgent::new(
                "Unknown".to_string(),
                "Unknown".to_string(),
                "Unknown".to_string(),
            ),
        }
    }
}
