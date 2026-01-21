use super::*;

pub struct CbArc<T> {
    cb: Arc<dyn Fn(T) + Send + Sync>,
}

impl<T> Clone for CbArc<T> {
    fn clone(&self) -> Self {
        Self { cb: self.cb.clone() }
    }
}

impl<T: 'static> CbArc<T> {
    pub fn new(f: Arc<dyn Fn(T) + Send + Sync>) -> CbArc<T> {
        CbArc { cb: f }
    }

    pub fn new0<F: Fn(T) + 'static + Send + Sync>(f: F) -> CbArc<T> {
        CbArc::new(Arc::new(f))
    }

    pub fn call(&self, t: T) {
        (self.cb.as_ref())(t)
    }

    pub fn map<U, F: Fn(U) -> Vec<T> + 'static + Send + Sync>(&self, f1: F) -> CbArc<U> {
        let cb = self.cb.clone();
        CbArc {
            cb: Arc::new(move |u| {
                let t = (f1)(u);
                for x in t {
                    (cb.as_ref())(x)
                }
            }),
        }
    }
}
