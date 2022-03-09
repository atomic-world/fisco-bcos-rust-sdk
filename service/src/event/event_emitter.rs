use std::sync::{Arc, RwLock};

pub struct Listener<'l, T> {
    name: String,
    listener: Arc<dyn Fn(&T) + Send + Sync + 'l>,
}

pub struct EventEmitter<'l, T> {
    pub listeners: Arc<RwLock<Vec<Listener<'l, T>>>>,
}

impl<'l, T> EventEmitter<'l, T> {
    pub fn new() -> EventEmitter<'l, T> {
        EventEmitter { listeners: Arc::new(RwLock::new(vec![])) }
    }

    pub fn on<F>(&self, name: &str, listener: F) where F: Fn(&T) + Send + Sync + 'l {
        let listeners_lock = self.listeners.clone();
        let mut listeners_write_lock = listeners_lock.write().unwrap();
        listeners_write_lock.push(Listener {
            name: name.to_owned(),
            listener: Arc::new(listener),
        });
    }

    pub fn emit(&self, name: &str, value: &T) {
        let listeners_lock = self.listeners.clone();
        let listeners_read_lock = listeners_lock.read().unwrap();
        for listener in listeners_read_lock.iter() {
            if listener.name.eq(name) {
                let callback = listener.listener.clone();
                callback(value);
            }
        }
    }

    pub fn remove(&self, name: &str) {
        let listeners_lock = self.listeners.clone();
        let mut listeners_write_lock = listeners_lock.write().unwrap();

        let mut  removed_indexes: Vec<usize> = vec![];
        for (index, listener) in listeners_write_lock.iter().enumerate() {
            if listener.name.eq(name) {
                removed_indexes.push(index);
            }
        }
        for removed_index in removed_indexes {
            listeners_write_lock.remove(removed_index);
        }
    }
}