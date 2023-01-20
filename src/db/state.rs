use super::{connector::Connector, aliases::Event};

pub struct State {
    pub connector: Connector,

    pub events: Vec<Event>,
    pub errors: Vec<()>,
}

impl State {
    pub fn new() -> Self {
        Self {
            connector: Connector::new(),
            events: vec![],
            errors: vec![],
        }
    }
}