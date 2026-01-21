use super::*;

pub struct MsgQueue<T> {
    items: Arc<Mutex<Vec<T>>>,
}

impl<T> Clone for MsgQueue<T> {
    fn clone(&self) -> Self {
        MsgQueue { items: self.items.clone() }
    }
}

impl<T> MsgQueue<T> {
    pub fn take_all(&self) -> Vec<T> {
        let mut t = self.items.lock().unwrap();
        let out = std::mem::replace(&mut *t, vec![]);
        out
    }

    pub fn extend(&self, items: Vec<T>) {
        let mut t = self.items.lock().unwrap();
        t.extend(items);
    }

    pub fn new() -> MsgQueue<T> {
        MsgQueue {
            items: Arc::new(Mutex::new(vec![])),
        }
    }
}
