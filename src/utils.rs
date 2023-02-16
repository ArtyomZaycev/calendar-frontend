use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn easy_spawn<F>(f: F) where F: Future<Output = ()> + Send + 'static {
    tokio::spawn(f);
}

#[cfg(target_arch = "wasm32")]
pub fn easy_spawn<F>(f: F) where F: Future<Output = ()> + 'static {
    wasm_bindgen_futures::spawn_local(f);
}