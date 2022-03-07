use std::cell::RefCell;
use std::sync::Arc;

pub struct Listener<'l, T> {
    name: String,
    listener: Arc<dyn Fn(&T) + Send + Sync + 'l>,
}

pub struct EventEmitter<'l, T> {
    pub listeners: RefCell<Vec<Listener<'l, T>>>,
}

impl<'l, T> EventEmitter<'l, T> {
    pub fn new() -> EventEmitter<'l, T> {
        EventEmitter { listeners: RefCell::from(vec![]) }
    }

    pub fn on<F>(&self, name: &str, listener: F) where F: Fn(&T) + Send + Sync + 'l {
        self.listeners.borrow_mut().push(Listener {
            name: name.to_owned(),
            listener: Arc::new(listener),
        });
    }

    pub fn emit(&self, name: &str, value: &T) {
        for listener in self.listeners.borrow().iter() {
            if listener.name.eq(name) {
                (listener.listener)(value);
            }
        }
    }

    pub fn remove(&self, name: &str) {
        let mut  removed_indexes: Vec<usize> = vec![];
        for (index, listener) in self.listeners.borrow().iter().enumerate() {
            if listener.name.eq(name) {
                removed_indexes.push(index);
            }
        }

       let mut listeners =  self.listeners.borrow_mut();
        for removed_index in removed_indexes {
            listeners.remove(removed_index);
        }
    }
}