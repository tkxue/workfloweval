#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use e_macro_include::*;

use proc_macro::Span;
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics,
    ImplGenerics, Path, PathSegment, TraitBound, TraitBoundModifier, TypeGenerics, TypeParam,
    TypeParamBound, Variant, WhereClause, parse_macro_input,
};

mod big_enum;
mod enum_flat;
mod tui_val;
mod x_jsdata;
mod x_netdata;

#[proc_macro_derive(Fake_X_Partial)]
pub fn proc_x_partial(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = quote! {};
    t.into()
}

#[proc_macro_derive(Fake_X_Draw_Config)]
pub fn proc_x_draw_config(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = quote! {};
    t.into()
}

#[proc_macro_derive(Fake_Rr_Val)]
pub fn proc_x_tui_draw(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = quote! {};
    t.into()
}

#[proc_macro_derive(JsData)]
pub fn proc_l_jsdata(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    x_jsdata::L_JsData::proc_l_jsdata(input)
}

#[proc_macro_derive(BigEnum)]
pub fn proc_big_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    big_enum::BigEnum::proc_big_enum(input)
}

#[proc_macro_derive(NetData)]
pub fn proc_s_netdata(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    x_netdata::L_NetData::proc_s_netdata(input)
}

#[proc_macro_derive(Enum_Flat)]
pub fn proc_enum_flat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_flat::Enum_Flat::proc_enum_flat(input)
}
