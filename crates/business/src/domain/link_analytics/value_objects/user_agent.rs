#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserAgent {
    pub browser: String,
    pub os: String,
    pub device: String,
}

impl UserAgent {
    pub fn new(browser: String, os: String, device: String) -> Self {
        Self {
            browser,
            os,
            device,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent_new() {
        let user_agent = UserAgent::new(
            "Chrome".to_string(),
            "Windows".to_string(),
            "Desktop".to_string(),
        );

        assert_eq!(user_agent.browser, "Chrome");
        assert_eq!(user_agent.os, "Windows");
        assert_eq!(user_agent.device, "Desktop");
    }
}
