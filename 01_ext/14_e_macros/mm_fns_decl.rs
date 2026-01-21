use super::*;

/*
#[macro_export]
macro_rules! mm_decl_singleton {
    {
            $struct_name:ident
            // $inst_name:ident ; $val:path ; $struct_name:ident ; $set:ident ; $get:ident
    } => {

        const _ : () = {
        static __fn: std::sync::OnceLock<Box<dyn Fn () -> $struct_name + Send + Sync>> = std::sync::OnceLock::new();
        static __val: std::sync::OnceLock<$struct_name> = std::sync::OnceLock::new();

        impl $struct_name {
            pub fn __set_fn(f: Box<dyn Fn () -> $struct_name + Send + Sync>) {
                __fn.set(f);
            }

            pub fn __get_val() -> &'static $struct_name {
                match __val.get() {
                    Some(x) => x,
                    None => {
                        let f = __fn.get().un(err!("Can not get 'impl_later' fn for {}", stringify!($struct_name)));
                        let v = f();
                        __val.set(v);
                        __val.get().un(err!("Can not get 'impl_later' val for {}", stringify!($struct_name)))
                    }
                }
            }
        }

        };

    }
}
 */

// ================================================

/*
#[macro_export]
macro_rules! mm_ol_ac {
    {
            $config_name:ident
    } => {

        const _ : () = {

            static _foo: std::sync::OnceLock< Arc< Actor_New_Fns<$config_name> > > = std::sync::OnceLock::new();

            impl $config_name {
                pub fn set_new_ac(v: Arc<Actor_New_Fns<$config_name>> ) {
                    _foo.set(v);
                }

                pub fn new_rc(
                    args: Actor_New_Args<$config_name>,
                ) -> IAc<$config_name> {
                    let t = _foo.get().un(err!("Can not find new_rc for {}", stringify!($config_name)));
                    (t._new_iac)(
                        args
                    )
                }

            }
        };
    }
}
*/

/*
#[macro_export]
macro_rules! mm_ol_async_ac {
    {
            $config_name:ident
    } => {

        const _ : () = {

            static _foo: std::sync::OnceLock< Arc< Actor_Async_New_Fns<$config_name> > > = std::sync::OnceLock::new();

            impl $config_name {
                pub fn set_async_new_ac(v: Arc<Actor_Async_New_Fns<$config_name>> ) {
                    _foo.set(v);
                }

                pub fn async_new_rc(
                    args: Actor_New_Args<$config_name>,
                ) -> Pin<Box<dyn Future<Output = IAc<$config_name>>>> {
                    let t = _foo.get().un(err!("Can not find new_rc for {}", stringify!($config_name)));
                    (t._async_new_iac)(
                        args
                    )
                }

            }
        };
    }
}
*/
