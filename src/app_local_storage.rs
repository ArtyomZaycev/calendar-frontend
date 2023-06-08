use crate::local_storage::{LocalStorage, LocalStorageTrait};

pub struct AppLocalStorage {
    local_storage: LocalStorage,
}

impl AppLocalStorage {
    pub fn new() -> Self {
        Self {
            local_storage: LocalStorage::new(),
        }
    }

    const KEY: &str = "key";
    pub fn get_key(&mut self) -> Option<Vec<u8>> {
        self.local_storage.get(Self::KEY)
    }
    pub fn store_key(&mut self, key: &Vec<u8>) {
        self.local_storage.put(Self::KEY, key);
    }
    pub fn clear_key(&mut self) {
        self.local_storage.clear(Self::KEY);
    }

    const USER_ID: &str = "user_id";
    pub fn get_user_id(&mut self) -> Option<i32> {
        self.local_storage.get(Self::USER_ID)
    }
    pub fn store_user_id(&mut self, user_id: i32) {
        self.local_storage.put(Self::USER_ID, &user_id);
    }
    pub fn clear_user_id(&mut self) {
        self.local_storage.clear(Self::USER_ID);
    }
}
