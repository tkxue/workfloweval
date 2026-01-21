use super::*;

#[macro_export]
macro_rules! mm_enum_usize{
    {
        $enum_name:ident
    }
        =>
    {
        impl Enum_Usize_T for $enum_name {
            const enum_len: usize = $enum_name::_enum_len;
            fn to_usize(&self) -> usize {
                self._to_usize()
            }
            fn from_usize(t: usize) -> Self {
                $enum_name::_from_usize(t)
            }
        }

        impl Box_Iter_T for $enum_name {
            fn box_iter() -> Box<dyn Iterator<Item = Self>>
            where Self: Sized {
                Box::new((0..Self::enum_len).map(|x| Self::from_usize(x)))
            }

        }


    }

}
