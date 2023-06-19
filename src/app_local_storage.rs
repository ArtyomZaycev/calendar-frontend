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

    const JWT: &str = "jwt";
    pub fn get_jwt(&mut self) -> Option<String> {
        self.local_storage.get(Self::JWT)
    }
    pub fn store_jwt(&mut self, jwt: String) {
        self.local_storage.put(Self::JWT, &jwt);
    }
    pub fn clear_jwt(&mut self) {
        self.local_storage.clear(Self::JWT);
    }
}
