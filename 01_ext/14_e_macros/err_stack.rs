use super::*;

#[derive(JsData, Clone)]
pub struct Err_Stack {
    pub orig: Rc<Err_Frame>,
    pub context: Vec<Rc<Err_Frame>>,
}

impl Err_Stack {
    pub fn new(t: Err_Frame) -> Err_Stack {
        Err_Stack {
            orig: Rc::new(t),
            context: vec![],
        }
    }

    pub fn attach(&mut self, t: Err_Frame) {
        self.context.push(Rc::new(t));
    }

    pub fn dump(&self) {
        Xdom_Logger::log_s(&self)
    }
}

pub type Res<T> = Result<T, Err_Stack>;

impl std::fmt::Debug for Err_Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Err_Stack Debug: {:?}]", self.orig)
    }
}

impl From<Err_Frame> for Err_Stack {
    fn from(value: Err_Frame) -> Self {
        Err_Stack {
            orig: Rc::new(value),
            context: vec![],
        }
    }
}

pub trait Err_Stack_T {
    fn attach_context(self, t: Err_Frame) -> Self;
}

impl<T> Err_Stack_T for Res<T> {
    fn attach_context(self, t: Err_Frame) -> Res<T> {
        let t = self.map_err(|mut x| {
            x.context.push(Rc::new(t));
            x
        });
        t
    }
}
