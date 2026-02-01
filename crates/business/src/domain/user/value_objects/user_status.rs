use std::fmt;

#[derive(Clone)]
pub enum UserStatus {
    Unverified,
    Verified,
    Locked,
    Disabled,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
            Self::Locked => "locked",
            Self::Disabled => "disabled",
        };

        write!(f, "{}", status_str)
    }
}
