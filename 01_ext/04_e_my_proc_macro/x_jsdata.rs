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
                        ident: proc_macro2::Ident::new("T_JsData_", proc_macro2::Span::call_site()),
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

pub struct L_JsData {}

impl L_JsData {
    pub fn proc_l_jsdata(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let DeriveInput {
            ident: outer_Ident,
            data,
            mut generics,
            ..
        } = parse_macro_input!(input);

        Util::inject_my_trait(&mut generics);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let output = match data {
            syn::Data::Struct(s) => match s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => {
                    let builder = JsData_Struct_Named::new(named);
                    builder.output(&outer_Ident, &impl_generics, &ty_generics, &where_clause)
                }

                syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let builder = JsData_Struct_Unnamed::new(unnamed);
                    builder.output(&outer_Ident, &impl_generics, &ty_generics, &where_clause)
                }

                syn::Fields::Unit => {
                    let builder = JsData_Struct_Named::new0();
                    builder.output(&outer_Ident, &impl_generics, &ty_generics, &where_clause)
                }
            },

            syn::Data::Enum(DataEnum { variants, .. }) => {
                let mut builder = JsData_Enum_Outer::new();

                let variants_len = variants.len();
                let label = if variants_len >= 255 {
                    quote! { u16 }
                } else {
                    quote! { u8 }
                };

                for (idx, part) in variants.iter().cloned().enumerate() {
                    let idx_token = if variants_len >= 255 {
                        let idx_u16 = idx as u16;
                        quote! { #idx_u16 }
                    } else {
                        let idx_u8 = idx as u8;
                        quote! { #idx_u8 }
                    };

                    let ident = part.ident;
                    let fields = part.fields;
                    let (write_js_code, read_js_code) = match fields {
                        Fields::Named(FieldsNamed { named, .. }) => {
                            let builder = JsData_Enum_Named::new(named);
                            builder.output(&outer_Ident, &ident, &idx_token)
                        }
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                            let builder = JsData_Enum_Unnamed::new(unnamed);
                            builder.output(&outer_Ident, &ident, &idx_token)
                        }
                        Fields::Unit => JsData_Enum_Outer::gen_unit(&outer_Ident, &ident, &idx_token),
                    };

                    builder.outer_write_js_code.push(write_js_code);
                    builder.outer_read_js_code.push(read_js_code);
                }

                builder.output(&label, &outer_Ident, &impl_generics, &ty_generics, &where_clause)
            }
            syn::Data::Union(DataUnion {
                fields: FieldsNamed { .. }, ..
            }) => {
                todo!("error in l_jsdata")
            }
        };
        output.into()
    }
}

pub struct JsData_Enum_Named {
    pub parts: Vec<syn::Field>,
}

impl JsData_Enum_Named {
    pub fn new0() -> JsData_Enum_Named {
        JsData_Enum_Named { parts: vec![] }
    }

    pub fn new(named: Punctuated<syn::Field, syn::token::Comma>) -> JsData_Enum_Named {
        let mut builder = JsData_Enum_Named::new0();

        for part in named.iter().cloned() {
            builder.parts.push(part);
        }

        builder
    }

    pub fn output(
        &self,
        outer_Ident: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
        idx_token: &proc_macro2::TokenStream,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let mut write_js_code = vec![];
        let mut read_js_code = vec![];
        let mut args = vec![];

        for part in self.parts.iter() {
            let x = part.clone().ident.unwrap();
            let ty = &part.ty;

            args.push(quote! { #x });
            write_js_code.push(quote! { #x.write_to_js(writer, transfers )?; });
            read_js_code.push(quote! { #x: <#ty as T_JsData_>::read_from_js(reader, transfers)?, });
        }

        let write_js = quote! {
        #outer_Ident::#ident { #(#args),* } => {
            #idx_token.write_to_js(writer, /* abs, */ transfers)?;
            #(#write_js_code);*;
            Ok(()) } };

        let read_js = quote! {
        #idx_token => {
        Ok( #outer_Ident::#ident {
                #(#read_js_code)* } ) } };

        (write_js, read_js)
    }
}

pub struct JsData_Enum_Outer {
    pub outer_write_js_code: Vec<proc_macro2::TokenStream>,
    pub outer_read_js_code: Vec<proc_macro2::TokenStream>,
}

impl JsData_Enum_Outer {
    pub fn new() -> JsData_Enum_Outer {
        JsData_Enum_Outer {
            outer_write_js_code: vec![],
            outer_read_js_code: vec![],
        }
    }

    pub fn gen_unit(
        outer_Ident: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
        idx_token: &proc_macro2::TokenStream,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        (
            quote! { #outer_Ident::#ident => { #idx_token.write_to_js(writer, transfers )?; Ok(()) } },
            quote! { #idx_token => { Ok(#outer_Ident::#ident) } },
        )
    }

    pub fn output(
        self,
        label: &proc_macro2::TokenStream,
        outer_Ident: &proc_macro2::Ident,
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> proc_macro2::TokenStream {
        let JsData_Enum_Outer {
            outer_write_js_code,
            outer_read_js_code,
        } = self;

        quote! {
            impl #impl_generics T_JsData_ for #outer_Ident #ty_generics #where_clause {
                fn write_to_js(&self, writer: T_JsData_Write,
                    transfers: &mut std::collections::VecDeque<wb::JsValue>,
                ) -> Result<(), L_JsData_Err> {
                    match &self {
                        #(#outer_write_js_code)*
                    }
                }

                fn read_from_js(reader: T_JsData_Read,
                    transfers: &mut std::collections::VecDeque<wb::JsValue>,
                ) -> Result<Self, L_JsData_Err> where Self: Sized {
                    let t: #label = <#label as T_JsData_>::read_from_js(reader, /* abs, */ transfers)?;
                    match t {
                        #(#outer_read_js_code)*
                        _ => { Err(L_JsData_Err::Illegal_Enum) }
                    }
                }
            }
        }
    }
}

pub struct JsData_Enum_Unnamed {
    pub parts: Vec<syn::Field>,
}

impl JsData_Enum_Unnamed {
    pub fn new0() -> JsData_Enum_Unnamed {
        JsData_Enum_Unnamed { parts: vec![] }
    }

    pub fn new(unnamed: Punctuated<syn::Field, syn::token::Comma>) -> JsData_Enum_Unnamed {
        let mut builder = JsData_Enum_Unnamed::new0();

        for part in unnamed.iter().cloned() {
            builder.parts.push(part.clone());
        }

        builder
    }

    pub fn output(
        &self,
        outer_Ident: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
        idx_token: &proc_macro2::TokenStream,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let mut write_js_code = vec![];
        let mut read_js_code = vec![];
        let mut anon_js_vars = vec![];

        for (idx, part) in self.parts.iter().enumerate() {
            let ty = &part.ty;
            let new_Id = proc_macro2::Ident::new(&format!("x_{}", idx), proc_macro2::Span::call_site());

            write_js_code.push(quote! { #new_Id.write_to_js(writer, transfers)?; });
            read_js_code.push(quote! { <#ty as T_JsData_>::read_from_js(reader, transfers)? });
            anon_js_vars.push(quote! { #new_Id });
        }

        let write_js = quote! {
            #outer_Ident::#ident ( #(#anon_js_vars),* ) => {
                #idx_token.write_to_js(writer, transfers)?;
                #(#write_js_code);*;
                Ok(())
            }
        };
        let read_js = quote! {
            #idx_token => {
                Ok(#outer_Ident::#ident( #(#read_js_code),* ))
            }
        };

        (write_js, read_js)
    }
}

pub struct JsData_Struct_Named {
    pub parts: Vec<syn::Field>,
}

impl JsData_Struct_Named {
    pub fn new0() -> JsData_Struct_Named {
        JsData_Struct_Named { parts: vec![] }
    }

    pub fn new(named: Punctuated<syn::Field, syn::token::Comma>) -> JsData_Struct_Named {
        let mut builder = JsData_Struct_Named::new0();

        for part in named.iter().cloned() {
            builder.parts.push(part.clone());
        }

        builder
    }

    pub fn output(
        self,
        outer_Ident: &proc_macro2::Ident,
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> proc_macro2::TokenStream {
        let JsData_Struct_Named { parts } = self;

        let mut write_js_code = vec![];
        let mut read_js_code = vec![];

        for part in parts.iter() {
            let x = part.clone().ident.unwrap();
            let ty = &part.ty;

            write_js_code.push(quote! { self.#x.write_to_js(writer, transfers)?; });
            read_js_code.push(quote! { #x: <#ty as T_JsData_>::read_from_js(reader, transfers)?, });
        }

        quote! {
            impl #impl_generics T_JsData_ for #outer_Ident #ty_generics #where_clause {
                fn write_to_js(
                        &self,
                        writer: T_JsData_Write,
                        transfers: &mut std::collections::VecDeque<wb::JsValue>,

                    )
                    -> Result<(), L_JsData_Err> {
                    #(#write_js_code);*;
                    Ok(())
                }

                fn read_from_js(
                        reader: T_JsData_Read,
                        transfers: &mut std::collections::VecDeque<wb::JsValue>,
                    )
                    -> Result<Self, L_JsData_Err> where Self: Sized {
                    Ok( #outer_Ident {
                            #(#read_js_code)*
                        } ) }

            }
        }
    }
}

pub struct JsData_Struct_Unnamed {
    pub parts: Vec<syn::Field>,
}

impl JsData_Struct_Unnamed {
    pub fn new0() -> JsData_Struct_Unnamed {
        JsData_Struct_Unnamed { parts: vec![] }
    }

    pub fn new(unnamed: Punctuated<syn::Field, syn::token::Comma>) -> JsData_Struct_Unnamed {
        let mut builder = JsData_Struct_Unnamed::new0();

        for part in unnamed.iter().cloned() {
            builder.parts.push(part.clone());
        }

        builder
    }

    pub fn output(
        self,
        outer_Ident: &proc_macro2::Ident,
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> proc_macro2::TokenStream {
        let mut write_js_code = vec![];
        let mut read_js_code = vec![];

        for (idx, part) in self.parts.iter().enumerate() {
            let ty = &part.ty;
            let idx2 = syn::Index::from(idx);

            write_js_code.push(quote! { (self.#idx2).write_to_js(writer, transfers)?; });
            read_js_code.push(quote! { <#ty as T_JsData_>::read_from_js(reader, transfers)?, });
        }

        quote! {
            impl #impl_generics T_JsData_ for #outer_Ident #ty_generics #where_clause {

                #[inline(always)]
                fn write_to_js(
                        &self,
                        writer: T_JsData_Write,
                        transfers: &mut std::collections::VecDeque<wb::JsValue>,
                    )
                    -> Result<(), L_JsData_Err> {
                    #(#write_js_code);*;
                    Ok(())
                }

                #[inline(always)]
                fn read_from_js(
                        reader: T_JsData_Read,
                        transfers: &mut std::collections::VecDeque<wb::JsValue>,
                    )
                    -> Result<Self, L_JsData_Err> where Self: Sized {
                    Ok( #outer_Ident (
                            #(#read_js_code)*
                        ) )
                    }

            }
        }
    }
}
