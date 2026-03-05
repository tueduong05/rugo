use business::domain::link_analytics::{
    services::UserAgentParser, value_objects::user_agent::UserAgent,
};
use woothee::parser::Parser;

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
            Some(data) => UserAgent {
                browser: data.name.to_string(),
                os: data.os.to_string(),
                device: data.category.to_string(),
            },

            None => UserAgent {
                browser: "Unknown".to_string(),
                os: "Unknown".to_string(),
                device: "Unknown".to_string(),
            },
        }
    }
}
