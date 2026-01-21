#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use e_include::*;
use e_jsdata::*;
use e_my_proc_macro::*;

mod err_msg;
mod err_stack;
mod gen_rr;
mod lazy_singleton;
mod mm_ensure;
mod mm_enum_slice;
mod mm_enum_usize;
mod mm_err;
mod mm_fns_decl;
mod mm_fwd;
mod mm_struct_funcs_wrapper;
mod startup;
mod xdom_logger;
pub use err_msg::*;
pub use err_stack::*;
pub use gen_rr::*;
pub use lazy_singleton::*;
pub use mm_ensure::*;
pub use mm_enum_slice::*;
pub use mm_enum_usize::*;
pub use mm_err::*;
pub use mm_fns_decl::*;
pub use mm_fwd::*;
pub use mm_struct_funcs_wrapper::*;
pub use startup::*;
pub use xdom_logger::*;

#[macro_export]
macro_rules! mm_id_enum_struct {
    {
        $struct_name:ident $struct_name_n:ident
        $enum_name:ident $enum_name_n:ident
        $trait_name:ident
        $ctor_trait_name:ident
        $ctor_trait_name_n:ident
        #[$meta_tag:meta]
        #[$meta_wbintf:meta]
        #[$meta_nbintf:meta]
        $( ( $struct_field_name:tt, $enum_arm_name:tt, $tag_Id:tt, $d:tt ); )*
    } => {

        $( #[$meta_tag] pub struct $tag_Id {} )*


        $(impl $trait_name for $tag_Id $d)*

        #[$meta_wbintf]
        pub struct $struct_name<T: $ctor_trait_name> {
            $( pub $struct_field_name : T::Ty<$tag_Id> ),*
        }

        #[$meta_nbintf]
        pub struct $struct_name_n<T: $ctor_trait_name_n> {
            $( pub $struct_field_name : T::TyN<$tag_Id> ),*
        }

        #[$meta_wbintf]
        pub enum $enum_name<T: $ctor_trait_name> {
            $( $enum_arm_name ( T::Ty<$tag_Id> ) ),*
        }

        #[$meta_nbintf]
        pub enum $enum_name_n<T: $ctor_trait_name_n> {
            $( $enum_arm_name ( T::TyN<$tag_Id> ) ),*
        }
    }
}

pub struct Enum_Finite<S: Eq + Hash + Copy> {
    pub v: Arc<Vec<S>>,
    pub hm: Arc<HashMap<S, usize>>,
}

impl<S: Eq + Hash + Copy> Enum_Finite<S> {
    pub fn new(v: Arc<Vec<S>>) -> Enum_Finite<S> {
        let mut hm = HashMap::new();
        for (i, x) in v.iter().enumerate() {
            hm.insert(*x, i);
        }
        Enum_Finite { v, hm: Arc::new(hm) }
    }
}

/*
impl Foo {
    fn _get_hash_map() -> Arc<HashMap<Foo, usize>> {
        // static __foo: Arc<HashMap<Foo, usize>> = Arc::new(HashMap::new());
        /*
        static __foo: std::sync::OnceLock<Arc<HashMap<Self, usize>>> = std::sync::OnceLock::new();
        match __foo.get() {
            Some(x) => x.clone(),
            None => {
                let t = Arc::new(Enum_Finite::new(vec![]));
                __foo.set(t.clone());
                t
            }
        }

         */
        // __foo.clone()
    }
}
*/

/*
pub struct Memoize_Util {}

impl Memoize_Util {
    pub fn ccmd<T>(f: Rc<dyn Fn() -> T>) {
        static __foo: std::sync::OnceLock<Arc<HashMap<T, usize>>> = std::sync::OnceLock::new();
    }
}

 */

pub trait Enum_Finite_T_: Sized + Eq + Hash + Copy {
    fn _get_vec() -> Arc<Vec<Self>>;
}

pub trait Get_HashMap_T_: Sized + Eq + Hash + Copy {}

/*
impl Get_Vec_T_ for Foo {
    fn _get_vec() -> Arc<Enum_Finite<Self>> {
    }
}

 */

pub trait Enum_Usize_T_ {
    const _enum_len: usize;

    fn _to_usize(&self) -> usize;
    fn _to_u32(&self) -> u32 {
        self._to_usize() as u32
    }

    fn _from_usize(t: usize) -> Self;

    fn _box_iter() -> Box<dyn Iterator<Item = Self>>
    where
        Self: Sized,
    {
        Box::new((0..Self::_enum_len).map(|x| Self::_from_usize(x)))
    }
}

pub trait Enum_Flat_T_: Sized + 'static {
    const __elems: &'static [Self];
}

pub trait Box_Iter_T {
    fn box_iter() -> Box<dyn Iterator<Item = Self>>
    where
        Self: Sized;
}

pub trait Vec_Iter_T {
    fn vec_iter() -> Vec<Self>
    where
        Self: Sized;
}

pub trait Enum_Usize_T {
    const enum_len: usize;

    fn to_usize(&self) -> usize;
    fn to_u32(&self) -> u32 {
        self.to_usize() as u32
    }

    fn from_usize(t: usize) -> Self;
}

pub trait Has_Tag<T> {
    const tag: T;
}

#[macro_export]
macro_rules! mm_box_iter {
    {
        $enum_Id:ident
    } => {

        impl Box_Iter for $enum_Id {
            fn box_iter() -> Box<dyn Iterator<Item = Self>> {
                use strum::IntoEnumIterator;
                Box::new($enum_Id::iter())
            }

        }

    }
}

// Note: not part of the public api.
#[doc(hidden)]
pub mod _private {
    pub use core::mem::forget;
    pub struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        #[cold]
        #[inline(never)]
        fn drop(&mut self) {
            panic!("Triggering abort via double panic (static initializer panicked).")
        }
    }
}
