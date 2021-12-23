pub struct  Listener<T> {
    name: String,
    listener: fn(&T),
}

pub struct EventEmitter<T> {
    pub listeners: Vec<Listener<T>>,
}

impl<T> EventEmitter<T> {
    pub fn new() -> EventEmitter<T> {
        EventEmitter { listeners: vec![] }
    }

    pub fn on(&mut self, name: &str, listener: fn(&T)) {
        self.listeners.push(Listener {
            name: name.to_owned(),
            listener,
        });
    }

    pub fn emit(&self, name: &str, value: &T) {
        for listener in &self.listeners {
            if listener.name.eq(name) {
                (listener.listener)(value);
            }
        }
    }

    pub fn remove(&mut self, name: &str) {
        let mut remove_listener_indexes: Vec<usize> = vec![];
        for (index, listener) in self.listeners.iter().enumerate() {
            if listener.name.eq(name) {
                remove_listener_indexes.push(index);
            }
        }
        for index in remove_listener_indexes.iter() {
            self.listeners.remove(*index);
        }
    }
}