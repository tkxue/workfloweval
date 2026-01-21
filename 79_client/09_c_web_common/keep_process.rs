use super::*;

pub struct Keep_Process<T> {
    inner: ArcState<Keep_Process_Inner<T>>,
}

impl<T> Keep_Process<T> {
    pub fn new() -> Keep_Process<T> {
        Keep_Process {
            inner: ArcState::new(Keep_Process_Inner::new()),
        }
    }

    pub fn process(&self, data: T) {
        /*
        if let Ok(mut x) = self.inner.try_borrow_mut() {
            x.process(data);
        }

         */
        todo!()
    }

    pub fn set_recv_func(&self, f: Rc<dyn Fn(T)>) {
        todo!()
        //self.inner.borrow_mut().set_recv_func(f);
    }
}

pub enum Keep_Process_Inner<T> {
    Func(Arc<dyn Fn(T) + Send + Sync>),
    Vec(Vec<T>),
}

impl<T> Keep_Process_Inner<T> {
    pub fn new() -> Keep_Process_Inner<T> {
        Keep_Process_Inner::Vec(vec![])
    }

    pub fn name(&self) -> &str {
        match self {
            Keep_Process_Inner::Func(_) => "func",
            Keep_Process_Inner::Vec(_) => "vec",
        }
    }

    fn process_helper(&mut self, t: T) {
        match self {
            Keep_Process_Inner::Func(f) => {
                f(t);
            }

            Keep_Process_Inner::Vec(v) => {
                v.push(t);
            }
        }
    }

    pub fn process(&mut self, data: T) {
        self.process_helper(data)
    }

    pub fn set_recv_func(&mut self, f: Rc<dyn Fn(T)>) {
        todo!()
        /*
        let t = std::mem::replace(self, Keep_Process_Inner::Func(f.clone()));
        match t {
            Keep_Process_Inner::Func(_) => {}
            Keep_Process_Inner::Vec(v) => {
                for x in v.into_iter() {
                    f(x)
                }
            }
        }

         */

        /*
        match f {
            None => match self {
                Keep_Process_Inner::Func(_) => *self = Keep_Process_Inner::Vec(vec![]),
                Keep_Process_Inner::Vec(_) => {}
            },

            Some(f) => {
            }
        }
        */
    }
}
