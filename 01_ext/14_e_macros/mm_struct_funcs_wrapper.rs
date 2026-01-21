#[allow(unused_imports)]
use super::*;

/*
pub struct YYY_MM_Struct_Funcs_Wrapper {}

#[macro_export]
macro_rules! mm_struct_funcs_wrapper {
    {
        $( #[$meta:meta] )*
        $vis:vis struct $struct_name:ident {
            $(pub $method_name:ident : &'static fn ($($tt:tt)+) $( -> $r_ty:ty)? ,)+
        }
        impl for $impl_type:ident;
    } => {
        $( #[$meta] )*
        $vis struct $struct_name {
            $(pub $method_name : &'static fn ($($tt)+) $( -> $r_ty)?,)+
        }

        impl $impl_type {
            $(mm_struct_funcs_wrapper! { @expand_impl ( $impl_type $struct_name $method_name $($tt)+)  $( -> $r_ty)? })+
        }

    };

    (@expand_impl ( $impl_type:ident $struct_name:ident $method_name:ident obj: Rc<$obj_type:ty>, funcs : $funcs_type:ty $(, $arg:ident : $arg_type:ty)* )  $( -> $r_ty:ty)? ) => {
        pub fn $method_name(self: Rc<Self>, funcs: $funcs_type, $($arg: $arg_type),*) $( -> $r_ty)? {
            (funcs.$method_name)(self, funcs, $($arg), *)
        }
    };


    (@expand_impl ( $impl_type:ident $struct_name:ident $method_name:ident obj: & $obj_type:ty, funcs : $funcs_type:ty $(, $arg:ident : $arg_type:ty) *) $( -> $r_ty:ty)? ) => {
        pub fn $method_name(&self, funcs: $funcs_type, $($arg: $arg_type), *) $( -> $r_ty)? {
            (funcs.$method_name)(self, funcs, $($arg), *)
        }
    };

    (@expand_impl ( $impl_type:ident $struct_name:ident $method_name:ident obj: (), funcs : $funcs_type:ty $(, $arg:ident : $arg_type:ty)* ) $( -> $r_ty:ty)? ) => {
        pub fn $method_name( funcs: $funcs_type, $($arg: $arg_type), *) $( -> $r_ty)? {
            (funcs.$method_name)((), funcs, $($arg), *)
        }
    };
}


 */
