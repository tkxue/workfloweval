use super::*;

use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, ImplGenerics, TypeGenerics, WhereClause, parse_macro_input};

struct Util {}
impl Util {
    pub fn inject_my_trait(generics: &mut Generics) {
        for x in &mut generics.params {
            match x {
                GenericParam::Type(t) => {
                    let mut x = Punctuated::new();
                    x.push(PathSegment {
                        ident: proc_macro2::Ident::new("T_BigEnum", proc_macro2::Span::call_site()),
                        arguments: Default::default(),
                    });
                    t.bounds.push(TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: TraitBoundModifier::None,
                        lifetimes: None,
                        path: Path {
                            leading_colon: None,
                            segments: x,
                        },
                    }));
                }
                GenericParam::Lifetime(_) => {}
                GenericParam::Const(_) => {}
            }
        }
    }
}

pub struct BigEnum {}

impl BigEnum {
    pub fn proc_big_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let DeriveInput {
            ident: outer_Ident,
            data,
            mut generics,
            ..
        } = parse_macro_input!(input);

        Util::inject_my_trait(&mut generics);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let output = match data {
            syn::Data::Enum(DataEnum { variants, .. }) => {
                let mut builder = BigEnum_Enum_Outer::new();

                let variants_len = variants.len();
                let label = if variants_len >= 255 {
                    quote! { u16 }
                } else {
                    quote! { u8 }
                };

                for (idx, part) in variants.iter().cloned().enumerate() {
                    let _idx_token = if variants_len >= 255 {
                        let idx_u16 = idx as u16;
                        quote! { #idx_u16 }
                    } else {
                        let idx_u8 = idx as u8;
                        quote! { #idx_u8 }
                    };

                    let ident = part.ident;
                    let fields = part.fields;
                    match fields {
                        Fields::Named(FieldsNamed { .. }) => {
                            // let builder = BigEnum_Enum_Named::new(named);
                            // builder.output(&outer_Ident, &ident, &idx_token)
                            todo!("big_enum does not handle named enums")
                        }
                        Fields::Unnamed(FieldsUnnamed { .. }) => {
                            // let builder = BigEnum_Enum_Unnamed::new(unnamed);
                            // builder.output(&outer_Ident, &ident, &idx_token)
                            todo!("big_enum does not handle unnamed enums")
                        }
                        Fields::Unit => {
                            builder.names.push(ident.to_string());
                        }
                    };
                }

                builder.output(&label, &outer_Ident, &impl_generics, &ty_generics, &where_clause)
            }
            _ => {
                todo!("big_enum only handles enums")
            }
        };
        output.into()
    }
}

pub struct BigEnum_Enum_Outer {
    pub names: Vec<String>,
}

impl BigEnum_Enum_Outer {
    pub fn new() -> BigEnum_Enum_Outer {
        BigEnum_Enum_Outer { names: vec![] }
    }

    pub fn output(
        self,
        _label: &proc_macro2::TokenStream,
        outer_Ident: &proc_macro2::Ident,
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> proc_macro2::TokenStream {
        let t = self.names.len() as u16;
        let s = self.names.iter().map(|x| quote! { #x }).collect::<Vec<_>>();

        quote! {


            /*
            impl #impl_generics T_NetData_ for #outer_Ident #ty_generics #where_clause {

                fn write_to_buf_<'b, W: std::io::Write>(&self, writer: &'b mut W) -> Result<(), L_NetData_Err> {
                    let x: u16 = self.to_u16_().inner;
                    x.write_to_buf_(writer)
                }

                fn read_from_buf_(reader: T_NetData_Read) -> Result<Self, L_NetData_Err> where
                Self: Sized {
                    let t = u16::read_from_buf_(reader)?;
                    Self::from_u16_(t).ok_or(L_NetData_Err::Illegal_Enum)
                }


            }

             */


            impl #impl_generics T_JsData_ for #outer_Ident #ty_generics #where_clause {
                #[inline(always)]
                fn write_to_js(
                    &self,
                    writer: T_JsData_Write,

                    transfers: &mut VecDeque<wb::JsValue>,
                ) -> Result<(), L_JsData_Err> {
                    u16::write_to_js(&self.to_u16_().inner, writer,  transfers)
                }

                #[inline(always)]
                fn read_from_js(
                    reader: T_JsData_Read,

                    transfers: &mut VecDeque<wb::JsValue>,
                ) -> Result<Self, L_JsData_Err>
                where
                    Self: Sized,
                {
                    let t = u16::read_from_js(reader,  transfers)?;
                    Self::from_u16_(t).ok_or(L_NetData_Err::Illegal_Enum)
                }
            }


            impl #impl_generics T_BigEnum_ for #outer_Ident #ty_generics #where_clause {
                const NUM_ARMS: u16 = #t;
                const names: &'static [&'static str] = &[#(#s),*];

                fn to_u16_(&self) -> U16_BigEnum<Self> {
                    U16_BigEnum::new(*self as u16)
                }

                fn from_u16_(x: u16) -> Option<Self> {
                    if x < Self::NUM_ARMS {
                        Some(
                            unsafe {
                                std::mem::transmute::<u16, Self>(x)
                            }
                        )

                    } else {
                        None
                    }
                }
            }


            impl Debug  for #outer_Ident #ty_generics #where_clause {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    f.write_str(Self::names[*self as u16 as usize])
                }
            }
        }
    }
}
