use std::{collections::HashMap, sync::Mutex};

use business::domain::{
    link::{
        entities::Link, error::LinkDomainError, repositories::LinkRepository,
        value_objects::short_code::ShortCode,
    },
    user::value_objects::user_id::UserId,
};

pub struct MockLinkRepository {
    short_codes: Mutex<HashMap<String, Link>>,
    ids: Mutex<HashMap<u64, Link>>,
    user_links: Mutex<HashMap<UserId, Vec<Link>>>,
    next_id: Mutex<u64>,
}

impl MockLinkRepository {
    pub fn new() -> Self {
        Self {
            short_codes: Mutex::new(HashMap::new()),
            ids: Mutex::new(HashMap::new()),
            user_links: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }
}

#[async_trait::async_trait]
impl LinkRepository for MockLinkRepository {
    async fn create(&self, link: &Link) -> Result<(), LinkDomainError> {
        let mut short_codes = self.short_codes.lock().unwrap();
        let mut ids = self.ids.lock().unwrap();
        let mut user_links = self.user_links.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let mut link_to_save = link.clone();

        if link_to_save.id.is_none() {
            link_to_save.id = Some(*next_id);
            *next_id += 1;
        }

        let code_str = link_to_save.short_code.to_string();
        let id = link_to_save.id.unwrap();

        short_codes.insert(code_str.clone(), link_to_save.clone());
        ids.insert(id, link_to_save.clone());

        if let Some(uid) = link_to_save.user_id {
            user_links
                .entry(uid)
                .or_insert_with(Vec::new)
                .push(link_to_save);
        }

        Ok(())
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Link>, LinkDomainError> {
        let ids = self.ids.lock().unwrap();
        Ok(ids.get(&id).cloned())
    }

    async fn find_by_short_code(
        &self,
        short_code: &ShortCode,
    ) -> Result<Option<Link>, LinkDomainError> {
        let short_codes = self.short_codes.lock().unwrap();
        Ok(short_codes.get(&short_code.to_string()).cloned())
    }

    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<Link>, LinkDomainError> {
        let user_links = self.user_links.lock().unwrap();
        Ok(user_links.get(&user_id).cloned().unwrap_or_default())
    }
}
