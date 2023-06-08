
pub trait LocalStorageTrait {
    fn new() -> Self;

    fn get_key(&mut self) -> Option<Vec<u8>>;

    fn store_key(&mut self, key: &[u8]);
}

#[cfg(not(target_arch = "wasm32"))]
mod storage {
    use super::LocalStorageTrait;

    pub struct LocalStorage {
        store: Option<scdb::Store>,
    }
    
    impl LocalStorageTrait for LocalStorage {
        fn new() -> Self {
            let store = scdb::Store::new(
                "calendar",
                Some(16),
                Some(1),
                Some(1),
                None,
                false
            );
            match store {
                Ok(store) => {
                    Self {
                        store: Some(store)
                    }
                },
                Err(error) => {
                    println!("Error while creating a local storage: {error:?}");
                    Self {
                        store: None
                    }
                },
            }
        }

        fn get_key(&mut self) -> Option<Vec<u8>> {
            self.store.as_mut().and_then(|store| {
                match store.get(&b"key"[..]) {
                    Ok(key) => key,
                    Err(error) => {
                        println!("Error while getting a key: {error:?}");
                        None
                    },
                }
            })
        }

        fn store_key(&mut self, key: &[u8]) {
            if let Some(store) = self.store.as_mut()     {
                if let Err(error) = store.set(&b"key"[..], key, None) {
                    println!("Error while storing a key: {error:?}");
                }
            }
        }
    }
}


#[cfg(target_arch = "wasm32")]
mod storage {
    use super::LocalStorageTrait;

    pub struct LocalStorage {
    
    }
    
    impl LocalStorageTrait for LocalStorage {
    }
}

pub use storage::*;