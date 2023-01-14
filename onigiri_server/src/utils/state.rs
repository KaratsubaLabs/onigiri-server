use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    sync::{Arc, Mutex},
};

/// queue of messages to pass to each device
#[derive(Default)]
pub struct DevPipe(MultiPipe);

impl Deref for DevPipe {
    type Target = MultiPipe;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

/// queue of messages to pass to each client
#[derive(Default)]
pub struct ClientPipe(MultiPipe);

impl Deref for ClientPipe {
    type Target = MultiPipe;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

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
        log::info!("[pipe] namespace: {}, data: {}", namespace, data);
    }

    pub fn read(&self, namespace: &str) -> Option<String> {
        let mut pipes = self.0.lock().unwrap();
        log::info!("[pipe] read from namespace {}", namespace);
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
