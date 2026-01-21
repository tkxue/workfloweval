use super::*;

#[derive(Debug)]
pub struct Rc2<T: Clone> {
    inner: Rc<RefCell<T>>,
}

impl<T: Clone> Clone for Rc2<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T: 'static + Clone> Rc2<T> {
    pub fn new(x: T) -> Rc2<T> {
        Rc2 {
            inner: Rc::new(RefCell::new(x)),
        }
    }

    pub fn replace(&self, x: T) {
        self.inner.replace(x);
    }

    pub fn get_cloned(&self) -> T {
        self.inner.deref().borrow().clone()
    }

    pub fn get<U>(&self, f: &dyn Fn(&T) -> U) -> U {
        let t = self.inner.deref().borrow();
        f(&t)
    }

    pub fn apply(&self, f: &dyn Fn(&mut T)) {
        let mut t = self.inner.deref().borrow_mut();
        f(&mut t);
    }
}

pub struct Rc2_cb<T: Clone> {
    inner: Rc<RefCell<T>>,
    cb: Rc<dyn Fn(T)>,
}

impl<T: Clone> Clone for Rc2_cb<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            cb: self.cb.clone(),
        }
    }
}

impl<T: 'static + Clone> Rc2_cb<T> {
    pub fn new(x: T, cb: Rc<dyn Fn(T)>) -> Rc2_cb<T> {
        Rc2_cb {
            inner: Rc::new(RefCell::new(x)),
            cb: cb.clone(),
        }
    }

    pub fn replace(&self, x: T) {
        self.inner.replace(x.clone());
        (self.cb.as_ref())(x);
    }

    pub fn get_cloned(&self) -> T {
        self.inner.deref().borrow().clone()
    }
}
