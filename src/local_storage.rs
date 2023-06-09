use serde::{de::DeserializeOwned, Serialize};
pub use storage::*;

pub trait LocalStorageTrait {
    fn new() -> Self;

    fn get<T>(&mut self, key: &str) -> Option<T>
    where
        T: DeserializeOwned;
    fn put<T>(&mut self, key: &str, data: &T)
    where
        T: Serialize;
    fn clear(&mut self, key: &str);
}

#[cfg(not(target_arch = "wasm32"))]
mod storage {
    use super::LocalStorageTrait;

    pub struct LocalStorage {
        store: Option<scdb::Store>,
    }

    impl LocalStorageTrait for LocalStorage {
        fn new() -> Self {
            let store = scdb::Store::new("calendar", Some(16), Some(1), Some(1), None, false);
            match store {
                Ok(store) => Self { store: Some(store) },
                Err(error) => {
                    println!("Error while creating a local storage: {error:?}");
                    Self { store: None }
                }
            }
        }

        fn get<T>(&mut self, key: &str) -> Option<T>
        where
            T: serde::de::DeserializeOwned,
        {
            self.store
                .as_mut()
                .and_then(|store| match store.get(key.as_bytes()) {
                    Ok(v) => v.and_then(|v| {
                        std::str::from_utf8(&v)
                            .ok()
                            .and_then(|s| serde_json::from_str(s).ok())
                    }),
                    Err(error) => {
                        println!(
                            "Error while retrieving data from native local storage: {error:?}"
                        );
                        None
                    }
                })
        }

        fn put<T>(&mut self, key: &str, data: &T)
        where
            T: serde::Serialize,
        {
            if let Some(store) = self.store.as_mut() {
                if let Ok(data) = serde_json::to_string(data) {
                    if let Err(error) = store.set(key.as_bytes(), data.as_bytes(), None) {
                        println!("Error while storing a key: {error:?}");
                    }
                }
            }
        }

        fn clear(&mut self, key: &str) {
            if let Some(store) = self.store.as_mut() {
                let _ = store.delete(key.as_bytes());
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod storage {
    use super::LocalStorageTrait;
    use gloo_storage::Storage;

    pub struct LocalStorage {}

    impl LocalStorageTrait for LocalStorage {
        fn new() -> Self {
            Self {}
        }

        fn get<T>(&mut self, key: &str) -> Option<T>
        where
            T: serde::de::DeserializeOwned,
        {
            gloo_storage::LocalStorage::get(key).ok()
        }

        fn put<T>(&mut self, key: &str, data: &T)
        where
            T: serde::Serialize,
        {
            let _ = gloo_storage::LocalStorage::set(key, data);
        }

        fn clear(&mut self, key: &str) {
            gloo_storage::LocalStorage::delete(key)
        }
    }
}
