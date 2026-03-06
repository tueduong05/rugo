use std::fmt;

use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        s.parse::<Uuid>().map(Self)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
