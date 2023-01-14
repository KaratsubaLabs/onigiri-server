use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

/// queue of messages to pass to each device
pub type DevPipe = MultiPipe;

/// queue of messages to pass to each client
pub type ClientPipe = MultiPipe;

/// Collection of pipes with name associated with each
// TODO make this generic
#[derive(Default)]
pub struct MultiPipe(Mutex<HashMap<String, VecDeque<String>>>);

impl MultiPipe {
    pub fn send(&self, namespace: &str, data: &str) {
        let mut pipe = self.0.lock().unwrap();
        if !pipe.contains_key(namespace) {
            pipe.insert(namespace.to_string(), VecDeque::new());
        }

        pipe.get_mut(namespace)
            .unwrap()
            .push_front(data.to_string());
    }

    pub fn read(&self, namespace: &str) -> Option<String> {
        let mut pipes = self.0.lock().unwrap();
        pipes.get_mut(namespace).and_then(|n| n.pop_back())
    }

    pub fn read_all(&self, namespace: &str) -> Vec<String> {
        let mut pipes = self.0.lock().unwrap();
        let pipe = match pipes.get_mut(namespace) {
            None => return Vec::new(),
            Some(p) => p,
        };

        // TODO pretty gross converting from VecDeque to Vec
        let ret = Vec::from_iter(pipe.into_iter().map(|x| x.clone()));
        pipe.clear();
        ret
    }

    /// purge empty pipes
    pub fn clean(&self) {
        unimplemented!()
    }
}
