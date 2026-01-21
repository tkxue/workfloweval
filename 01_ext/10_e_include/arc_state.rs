use super::*;

#[derive(Debug, Default)]
pub struct ArcState<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> Clone for ArcState<T> {
    fn clone(&self) -> Self {
        ArcState {
            inner: self.inner.clone(),
        }
    }
}

impl<T> ArcState<T> {
    pub fn new(t: T) -> ArcState<T> {
        ArcState {
            inner: Arc::new(RwLock::new(t)),
        }
    }

    pub fn update<U, F: FnOnce(&mut T) -> U>(&self, f: F) -> U {
        let mut t = self.inner.write().unwrap();
        let out = f(&mut t);
        out
    }

    pub fn read<U, F: FnOnce(&T) -> U>(&self, f: F) -> U {
        let t = self.inner.read().unwrap();
        let out = f(&t);
        out
    }
}
