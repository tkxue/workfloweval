use super::*;

/*
pub struct Lazy_Singleton<T: Clone> {
    name: &'static str,
    val: std::cell::OnceCell<T>,
    func: std::cell::OnceCell<Arc<dyn Fn() -> T>>,
}

impl<T: Clone> Lazy_Singleton<T> {
    pub const fn new(s: &'static str) -> Lazy_Singleton<T> {
        Lazy_Singleton {
            name: s,
            val: std::cell::OnceCell::new(),
            func: std::cell::OnceCell::new(),
        }
    }

    pub fn set_fn(&self, f: Arc<dyn Fn() -> T>) {
        match self.func.set(f) {
            Ok(_) => {}
            Err(_) => {
                damn_it!("Once_Init :: Already set func for {}", self.name);
            }
        }
    }

    pub fn get_val(&self) -> T {
        match self.val.get() {
            Some(x) => x.clone(),
            None => {
                let f = match self.func.get() {
                    Some(x) => x,
                    None => {
                        damn_it!("Once_Init :: Never registered func for: {:?}", self.name)
                    }
                };
                let _ = self.val.set((f.as_ref())());
                self.val.get().unwrap().clone()
            }
        }
    }
}

#[macro_export]
macro_rules! mm_lazy_singleton {
    {
        $struct_name:ident ; $val:path ; $set_fn:ident ; $get_val:ident ;
    } => {

        const _ : () = {

            thread_local! {
                static __foo: RefCell<Lazy_Singleton< $val >> = RefCell::new(Lazy_Singleton::new(
                    concat!("Lazy_Singleton on ", stringify!($struct_name) , " of type ", stringify!($val)) ) );
            }


            impl $struct_name {
                pub fn $set_fn(v: Arc<dyn Fn() -> $val  >) {
                    __foo.with_borrow(|x|  x.set_fn(v));
                }

                pub fn $get_val() -> $val {
                    __foo.with_borrow(|x| x.get_val().clone())
                }
            }


        };
    }
}


 */
