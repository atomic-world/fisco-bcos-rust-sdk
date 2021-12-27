pub struct Listener<'l, T> {
    name: String,
    listener: Box<dyn Fn(&T) + 'l>,
}

pub struct EventEmitter<'l, T> {
    pub listeners: Vec<Listener<'l, T>>,
}

impl<'l, T> EventEmitter<'l, T> {
    pub fn new() -> EventEmitter<'l, T> {
        EventEmitter { listeners: vec![] }
    }

    pub fn on<F>(&mut self, name: &str, listener: F) where F: Fn(&T) + 'l {
        self.listeners.push(Listener {
            name: name.to_owned(),
            listener: Box::new(listener),
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