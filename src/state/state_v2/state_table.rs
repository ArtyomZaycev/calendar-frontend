use super::requests_holder::RequestsHolder;

pub struct StateTable<T> {
    data: Vec<T>,
    requests: RequestsHolder,
}

impl<T> StateTable<T> {
    pub(super) fn new() -> Self {
        Self {
            data: Vec::new(),
            requests: RequestsHolder::new(),
        }
    }
}
