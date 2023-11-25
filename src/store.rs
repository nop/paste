use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use crate::error::{Error, Result};

pub trait Store {
    type Key;

    async fn create(&mut self, value: String) -> Result<Self::Key>;
    async fn read(&self, key: &Self::Key) -> Option<String>;
    async fn delete(&mut self, key: &Self::Key) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct ModelController {
    store: Arc<RwLock<HashMap<usize, String>>>,
    counter: Arc<Mutex<usize>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        let store = Arc::default();
        Ok(Self {
            store,
            counter: Arc::default(),
        })
    }
}

impl Store for ModelController {
    type Key = usize;

    async fn create(&mut self, value: String) -> Result<Self::Key> {
        let mut store = self.store.write().unwrap();

        // FIXME: This is slow! Use a key that is a hash of the content and just check against keys.
        // assert!(!self.contains_key(&id));
        if let Some((existing_id, _)) = store.iter().find(|(_, v)| *v == &value) {
            return Err(Error::PasteExists(*existing_id));
        }

        let mut counter = self.counter.lock().unwrap();
        store.insert(*counter, value);
        let id = *counter;
        *counter += 1;
        Ok(id)
    }

    async fn read(&self, key: &Self::Key) -> Option<String> {
        let store = self.store.read().unwrap();
        store.get(key).cloned()
    }

    async fn delete(&mut self, key: &Self::Key) -> Result<()> {
        let mut store = self.store.write().unwrap();
        store
            .remove_entry(key)
            .map_or(Err(Error::PasteNotFound), |_| Ok(()))
    }
}
