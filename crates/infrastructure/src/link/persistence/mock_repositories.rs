use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use business::domain::{
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
    user::value_objects::user_id::UserId,
};

pub struct MockLinkRepository {
    pub links: Arc<Mutex<HashMap<String, Link>>>,
}

impl MockLinkRepository {
    pub fn new() -> Self {
        Self {
            links: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl LinkRepository for MockLinkRepository {
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError> {
        let mut links = self.links.lock().unwrap();
        links.insert(link.short_code.to_string(), link.clone());
        Ok(())
    }

    async fn find_by_short_code(&self, short_code: &ShortCode) -> Result<Link, LinkDomainError> {
        let links = self.links.lock().unwrap();
        links
            .get(&short_code.to_string())
            .cloned()
            .ok_or(LinkDomainError::InvalidShortCode)
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Link>, LinkDomainError> {
        let links = self.links.lock().unwrap();

        let user_links: Vec<Link> = links
            .values()
            .filter(|link| link.user_id.as_ref() == Some(user_id))
            .cloned()
            .collect();

        Ok(user_links)
    }
}
