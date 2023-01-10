use proc_macro::TokenStream;
//use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use syn::{parse::{Parse, Parser}, parse_macro_input, DeriveInput, /*AttributeArgs,*/};

#[proc_macro_attribute]
pub fn custom_derive(_raw_args: TokenStream, input: TokenStream) -> TokenStream {
//    let args = parse_macro_input!(raw_args as AttributeArgs);
    let ast = parse_macro_input!(input as DeriveInput);
//    let mut ast_cloned = ast.clone();
    let (actual_fs, synthetic_fs_with_tokens);
    let prefix_path: syn::Path = syn::Path::parse.parse2(quote!{ custom_derive }).expect("PREFIX failed to unwrap : BUG :");
    match &ast.data {
        syn::Data::Struct(ref s) => match &s.fields {
            syn::Fields::Named(fields) => {
                let (actual_fs_, synthetic_fs_) = fields.named.iter().fold((vec![], vec![]), |mut acc, f| {
                    match f.attrs.iter().enumerate().find(|(_, attr)| attr.path == prefix_path) {
                        Some((idx, tattrs)) => {
                            let mut f_cloned = f.clone();
                            f_cloned.attrs.remove(idx);
                            acc.1.push((f_cloned, tattrs.tokens.clone()));
                        },
                        None => {
                            acc.0.push(f.clone());
                        }
                    };
                    acc
                });
                actual_fs = actual_fs_;
                synthetic_fs_with_tokens = synthetic_fs_;
            },
            _ => {
                panic!("only Named Structs are supported");
            }
        },
        _ => {
            panic!("unsupported type");
        }
    };

    let struct_ident = &ast.ident;
    let struct_vis = &ast.vis;
    let struct_attrs = &ast.attrs;
    let struct_generics = &ast.generics; // NOT SUPPORTED since they must be added to impl<'de>

//    let struct_ident_internal: TokenStream2 = {
//        let tmp = format!("{}_internal", struct_ident);
//        tmp.parse().unwrap()
//    };
    let struct_ident_internal = format_ident!("{}Internal", struct_ident);

    let synthetic_fs: Vec<_> = synthetic_fs_with_tokens.iter().map(|(f, _)| f.clone()).collect();

    let actual_fs_idents: Vec<_> = actual_fs.iter().map(|f| f.ident.clone().unwrap()).collect();
    let synthetic_fs_idents: Vec<_> = synthetic_fs.iter().map(|f| f.ident.clone().unwrap()).collect();
    let synthetic_fs_tokens: Vec<_> = synthetic_fs_with_tokens.iter().map(|(_, ts)| ts.clone()).collect();

    let output = quote! {

        #(#struct_attrs)*
        #struct_vis struct #struct_ident #struct_generics {
            #(#actual_fs,)*
            #(#synthetic_fs,)*
        }

        impl<'de> Deserialize<'de> for #struct_ident #struct_generics {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>
            {
                #(#struct_attrs)*
                #[derive(Deserialize)]
                struct #struct_ident_internal #struct_generics {
                    #(#actual_fs,)*
                }

                let si: #struct_ident_internal = Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    #(#synthetic_fs_idents: #synthetic_fs_tokens,)*
                    #(#actual_fs_idents: si.#actual_fs_idents,)*
                })
            }
        }
    };

    output.into()
}
