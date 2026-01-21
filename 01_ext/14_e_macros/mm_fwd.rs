use super::*;

/*
#[macro_export]
macro_rules! mm_fwd_decl {
    {
        $fwd_name:ident ; $trait_name:ident

    } => {

        #[derive(Clone)]
        pub struct $fwd_name {
            pub inner: Rc<dyn $trait_name>,
        }

    }
}

#[macro_export]
macro_rules! mm_fwd_impl {
    {
            $fwd_name:path ; $struct_name:ident
    } => {

        /*
        impl Into_Fwd_Rc_Any_T for $struct_name {
            fn into_opq_rc_any(self: Rc<Self>) -> Rc<dyn Any> {
                self
            }
        }

         */


        impl $struct_name {
            /*
            pub fn rc_from_fwd(t: $fwd_name) -> Rc<Self> {
                t.inner.into_opq_rc_any().downcast::<$struct_name>().un(err!(""))
            }

             */

            pub fn into_fwd(self) -> $fwd_name {
                $fwd_name {
                    inner: Rc::new(self)
                }
            }

            pub fn rc_into_fwd(self: Rc<Self>) -> $fwd_name {
                $fwd_name {
                    inner: self
                }
            }
        }
    }
}
*/

pub trait Into_Rc_Any_T {
    fn into_opq_rc_any(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any>;
}

pub trait Into_Ref_Any_T {
    fn to_opq_ref_any<'a>(&'a self) -> &'a dyn std::any::Any;
}
