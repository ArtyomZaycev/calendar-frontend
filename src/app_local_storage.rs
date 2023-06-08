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

    pub fn get_key(&mut self) -> Option<Vec<u8>> {
        self.local_storage.get("key")
    }
    pub fn store_key(&mut self, key: &Vec<u8>) {
        self.local_storage.put("key", key);
    }
    pub fn clear_key(&mut self) {
        self.local_storage.clear("key");
    }
}
