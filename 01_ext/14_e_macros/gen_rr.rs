use super::*;

#[macro_export]
macro_rules! gen_rr_s {
    (
        $other:ident ;
        $sv:vis struct $name:ident {
            $( $( #[$attrs:meta] )* $fv:vis $field_name:ident : $field_type:ty, )*
        }
    ) => {
        #[derive(Rr_Val)]
        $sv struct $name {
            $( $( #[$attrs] )* $fv $field_name : $field_type, )*
        }

        $sv struct $other<'a> {
            $( $( #[$attrs] )* $fv $field_name : Rr_Field<'a, $field_type>, )*
        }
    };
    (
        $other:ident ;
        $sv:vis struct $name:ident (
            $( $fv:vis $field_type:ty, )*
        );
    ) => {
        #[derive(Rr_Val)]
        $sv struct $name (
            $( $fv $field_type, )*
        );

        $sv struct $other<'a> (
            $( $fv Rr_Field<'a, $field_type>, )*
        );
    };
}

#[macro_export]
macro_rules! gen_rr_e {
    (
        $name_rr:ident ;
        $ev:vis enum $enum_name:ident {
            $(
            $v_name:ident
            $( ( $( $ft0:ty, )* ))?
            $({ $( $a_name:ident : $ft1:ty, )* })?
            ,
            )*
        }
    ) => {
        #[derive(Rr_Val)]
        $ev enum $enum_name {
            $(
            $v_name
            $( ( $( $ft0, )* ))?
            $({ $( $a_name : $ft1, )* })?
            ,
            )*
        }

        $ev enum $name_rr<'a> {
            $(
            $v_name
            $( ( $( Rr_Field<'a, $ft0>, )* ))?
            $({ $( $a_name : Rr_Field<'a, $ft1>, )* })?
            ,
            )*
        }
    }
}
