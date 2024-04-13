use std::sync::{Arc, Mutex};

use itertools::Itertools;

use super::State;

pub type StateChecker = Box<dyn Fn(&State) -> Option<StateExecutor> + Send>;
pub type StateExecutor = Box<dyn FnOnce(&mut State) + Send>;

pub struct StateUpdater {
    // Checker checks if state should be updated
    // Executor performs necessary actions
    // Needs to be separated because State needs to be populated 1 frame after response is received
    checkers: Arc<Mutex<Vec<StateChecker>>>,
    executors: Arc<Mutex<Vec<StateExecutor>>>,
}

impl StateUpdater {
    fn new() -> Self {
        Self {
            checkers: Arc::new(Mutex::new(Vec::new())),
            executors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get() -> &'static Self {
        use std::sync::OnceLock;

        static DATA: OnceLock<StateUpdater> = OnceLock::new();
        DATA.get_or_init(|| StateUpdater::new())
    }

    // Used for checking for pending requests, but it's not really correct
    pub fn any_checkers(&self) -> bool {
        self.checkers
            .try_lock()
            .is_ok_and(|checkers| checkers.len() > 0)
    }

    pub fn any_executors(&self) -> bool {
        self.executors
            .try_lock()
            .is_ok_and(|executors| executors.len() > 0)
    }

    pub fn push_checker(&self, checker: StateChecker) {
        self.checkers.lock().unwrap().push(checker);
    }

    pub fn push_executor(&self, executor: StateExecutor) {
        self.executors.lock().unwrap().push(executor);
    }

    pub fn update(&self, state: &mut State) {
        let executors = self.executors.lock().unwrap().drain(..).collect_vec();
        executors.into_iter().for_each(|executor| executor(state));
        let mut new_executors = vec![];
        let checkers = self.checkers.lock().unwrap().drain(..).collect_vec();
        let mut remained_checkers = checkers
            .into_iter()
            .filter_map(|checker: StateChecker| {
                if let Some(executor) = checker(&state) {
                    new_executors.push(executor);
                    None
                } else {
                    Some(checker)
                }
            })
            .collect_vec();
        self.checkers.lock().unwrap().append(&mut remained_checkers);
        self.executors.lock().unwrap().append(&mut new_executors);
    }
}
