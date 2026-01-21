use super::*;

/*
pub struct Rr_Val {}

impl Rr_Val {
    /*
    pub fn new() -> Rr_Val {
        Rr_Val {}
    }

     */

    pub fn output(self, outer_Ident: &proc_macro2::Ident, vs: &Vec<Ident>) -> proc_macro2::TokenStream {
        let num_Id = (0..vs.len()).map(|x| quote! { #x }).collect::<Vec<_>>();
        let first = vs.first().unwrap().clone();
        let len = vs.len();
        quote! {
            impl Rr_Val_T_ for #outer_Ident {
                const _enum_len: usize = #len;
                fn _to_usize(&self) -> usize {
                    match self {
                        #( #outer_Ident::#vs  =>  #num_Id, )*
                    }
                }

                fn _from_usize(t: usize) -> Self {
                    match t {
                        #( #num_Id => #outer_Ident::#vs, )*
                        _ => #outer_Ident::#first ,
                    }
                }
            }
        }
    }

    pub fn proc_rr_val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let file: syn::File = syn::parse2(input.into()).unwrap();
        let mut parts = Rast_File {
            structs: vec![],
            enums: vec![],
        };
        parts.parse_stream(file);
        match parts.structs.get(0) {
            Some(x) => {
                return e_rast_gen_rr_val::gen_struct(&x).into();
            }
            None => match parts.enums.get(0) {
                Some(x) => {
                    return e_rast_gen_rr_val::gen_enum(&x).into();
                }
                None => {
                    return quote! {}.into();
                }
            },
        }
    }
}


 */
