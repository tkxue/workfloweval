#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use e_api::*;

mod cb_arc;
mod id_g_actor;
mod id_proc;
mod incoming_ports;
mod keep_process;
mod msg_senders;
pub mod my_ffi;
mod post_office;
mod rc2;
mod web_root;
mod xdom;
mod xdoma_message_port;
mod xos_msg;
pub use cb_arc::*;
pub use id_g_actor::*;
pub use id_proc::*;
pub use incoming_ports::*;
pub use keep_process::*;
pub use msg_senders::*;
pub use post_office::*;
pub use rc2::*;
pub use web_root::*;
pub use xdom::*;
pub use xdoma_message_port::*;
pub use xos_msg::*;

mod msg_po;
pub use msg_po::*;

// ===============================

/*
pub trait Actor_Config_T<'a> {
    // to pass &, Remote_In needs 'a
    type Local_In;
    type Remote_In: To_Msg_Po_T;
    type Args_New;
}

pub struct WeakAc<C: Actor_Config_T<'static>> {
    inner: Weak<dyn Impl_Interior_Mut_Actor_T<C>>,
    msgs: Weak<RefCell<Vec<ArcAc_Msg<C>>>>,
}

pub struct Arc_Ac<C: Actor_Config_T<'static>> {
    inner: Rc<dyn Impl_Interior_Mut_Actor_T<C>>,
    msgs: Rc<RefCell<Vec<ArcAc_Msg<C>>>>,
}

impl<C: Actor_Config_T<'static>> Arc_Ac<C> {
    pub fn queue_local(&self, t: <C as Actor_Config_T<'static>>::Local_In) {
        self.msgs.borrow_mut().push(ArcAc_Msg::Local(t));
    }

    pub fn queue_remote(&self, t: <C as Actor_Config_T<'static>>::Remote_In) {
        self.msgs.borrow_mut().push(ArcAc_Msg::Remote(t));
    }

    pub fn tick_once(&self, label: &'static str) {
        let v = self.msgs.replace(vec![]);
        for x in v {
            match x {
                ArcAc_Msg::Local(x) => self.inner.handle_local(label, &self, x),
                ArcAc_Msg::Remote(x) => self.inner.handle_remote(label, &self, x),
            }
        }
    }

    pub fn tick_loop(&self, label: &'static str) {
        loop {
            let v = self.msgs.replace(vec![]);
            if v.is_empty() {
                return;
            }
            for x in v {
                match x {
                    ArcAc_Msg::Local(x) => self.inner.handle_local(label, &self, x),
                    ArcAc_Msg::Remote(x) => self.inner.handle_remote(label, &self, x),
                }
            }
        }
    }

    pub fn new(x: Rc<dyn Impl_Interior_Mut_Actor_T<C>>) -> Self {
        Self {
            inner: x,
            msgs: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn new0<T: Impl_Actor_T<C> + 'static>(label: String, x: T) -> Self {
        Self {
            inner: Rc::new(Dbg_RefCell::new(label, x)) as _,
            msgs: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn to_weak(&self) -> WeakAc<C> {
        WeakAc {
            inner: Rc::downgrade(&self.inner),
            msgs: Rc::downgrade(&self.msgs),
        }
    }
}

impl<C: Actor_Config_T<'static>> Clone for Arc_Ac<C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            msgs: self.msgs.clone(),
        }
    }
}


pub trait Impl_Interior_Mut_Actor_T<C: Actor_Config_T<'static>> {
    fn handle_local(&self, label: &'static str, ra: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Local_In);

    fn handle_remote(&self, label: &'static str, ra: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Remote_In);
}

pub enum ArcAc_Msg<T: Actor_Config_T<'static>> {
    Local(<T as Actor_Config_T<'static>>::Local_In),
    Remote(<T as Actor_Config_T<'static>>::Remote_In),
}

pub trait Actor_New_T<C: Actor_Config_T<'static>>: Impl_Actor_T<C> + Sized + 'static {
    fn new_iac(args: Actor_New_Args<C>) -> Arc_Ac<C>;

    fn make_fns() -> Actor_New_Fns<C> {
        Actor_New_Fns {
            _new_iac: Arc::new(|args| Self::new_iac(args)),
        }
    }

    fn __make_fn_new() -> Arc<dyn Fn() -> Arc<dyn Fn(Actor_New_Args<C>) -> Arc_Ac<C> + Send + Sync> + Send + Sync> {
        Arc::new(|| Arc::new(|x| Self::new_iac(x)))
    }
}

pub trait Impl_Actor_T<C: Actor_Config_T<'static>>: Sized + 'static {
    fn handle_local(&mut self, ra: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Local_In);

    fn handle_remote(&mut self, ra: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Remote_In);

    fn id_gactor() -> Option<Id_GActor>;
}

pub struct Actor_New_Args<C: Actor_Config_T<'static>> {
    pub init: <C as Actor_Config_T<'static>>::Args_New,
}

pub struct Actor_New_Fns<C: Actor_Config_T<'static>> {
    pub _new_iac: Arc<dyn Fn(Actor_New_Args<C>) -> Arc_Ac<C> + Send + Sync>,
}

#[derive(JsData, Clone)]
pub enum RAc_H_Index {
    Hide_Sound_Div,
    Init,
}

impl To_Msg_Po_T for RAc_H_Index {
    fn id_proc() -> Id_Proc {
        Id_Proc::H_Index
    }

    fn id_gactor() -> Id_GActor {
        Id_GActor::H_Index
    }
}
*/


pub struct Dbg_RefCell<T> {
    inner: RefCell<T>,
    old_label: Cell<&'static str>,
    name: String,
}

impl<T> Dbg_RefCell<T> {
    pub fn new(name: String, t: T) -> Dbg_RefCell<T> {
        Dbg_RefCell {
            inner: RefCell::new(t),
            old_label: Cell::new(""),
            name,
        }
    }

    pub fn borrow_mut(&self, label: &'static str) -> std::cell::RefMut<'_, T> {
        match self.inner.try_borrow_mut() {
            Ok(x) => {
                self.old_label.set(label);
                x
            }
            Err(_e) => {
                damn_it!(
                    "Borrow Failure\n  name: {:?}\n  old label: {:?}\n  new label: {:?}\n",
                    self.name,
                    self.old_label.get(),
                    label
                );
            }
        }
    }

    pub fn borrow(&self, label: &'static str) -> std::cell::Ref<'_, T> {
        match self.inner.try_borrow() {
            Ok(x) => {
                self.old_label.set(label);
                x
            }
            Err(e) => {
                damn_it!(
                    "Borrow Failure\n  name: {:?}\n  old label: {:?}\n  new label: {:?}\n",
                    self.name,
                    self.old_label.get(),
                    label
                );
            }
        }
    }

    pub fn replace(&self, label: &'static str, mut t: T) -> T {
        let mut g = self.borrow_mut(label);
        std::mem::swap(&mut t, g.deref_mut());
        t
    }
}

/*
impl<C: Actor_Config_T<'static>, T: Impl_Actor_T<C>> Impl_Interior_Mut_Actor_T<C> for Dbg_Mutex<T> {
    fn handle_local(&self, label: &'static str, sa: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Local_In) {
        let mut l = self.lock(label);
        l.handle_local(sa, t)
    }

    fn handle_remote(&self, label: &'static str, sa: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Remote_In) {
        let mut l = self.lock(label);
        l.handle_remote(sa, t)
    }
}

 */

/*
impl<C: Actor_Config_T<'static>, T: Impl_Actor_T<C>> Impl_Interior_Mut_Actor_T<C> for Dbg_RefCell<T> {
    fn handle_local(&self, label: &'static str, ra: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Local_In) {
        self.borrow_mut(label).handle_local(ra, t)
    }

    fn handle_remote(&self, label: &'static str, ra: &Arc_Ac<C>, t: <C as Actor_Config_T<'static>>::Remote_In) {
        self.borrow_mut(label).handle_remote(ra, t)
    }
}

 */

#[derive(Clone)]
pub struct Js_Parse_Error {
    pub error: String,
    pub v: wb::JsValue,
}
