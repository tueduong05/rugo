use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetLinkRequest {
    pub password: Option<String>,
}
