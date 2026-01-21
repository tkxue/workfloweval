use super::*;

pub struct Enum_Flat {}

impl Enum_Flat {
    pub fn new() -> Enum_Flat {
        Enum_Flat {}
    }

    pub fn output(self, outer_ident: &proc_macro2::Ident, vs: &Vec<Ident>) -> proc_macro2::TokenStream {
        quote! {
            impl Enum_Flat_T_ for #outer_ident {
                const __elems: &'static [Self] = &[
                        #( #outer_ident::#vs ),*
                ];
            }
        }
    }

    pub fn proc_enum_flat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let DeriveInput { ident: outer_ident, data, .. } = parse_macro_input!(input);

        let output = match data {
            syn::Data::Enum(DataEnum { variants, .. }) => {
                let builder = Enum_Flat::new();
                let mut vs = vec![];
                for x in variants.iter() {
                    match x.fields {
                        Fields::Unit => {}
                        Fields::Named(_) => {
                            todo!("named field")
                        }
                        Fields::Unnamed(_) => {
                            todo!("unnamed field")
                        }
                    }
                    vs.push(x.ident.clone());
                }
                let t = builder.output(&outer_ident, &vs);
                t
            }
            _ => {
                todo!("error in x_enum_Id")
            }
        };
        output.into()
    }
}
