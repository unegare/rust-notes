use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse::{Parse, Parser}, parse_macro_input, DeriveInput, /*FieldsNamed, AttributeArgs,*/ NestedMeta, Meta};

#[proc_macro_attribute]
pub fn custom_derive_deserialize(_raw_args: TokenStream, input: TokenStream) -> TokenStream {
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

    let struct_ident_internal = format_ident!("{}Internal", struct_ident);

    let synthetic_fs: Vec<_> = synthetic_fs_with_tokens.iter().map(|(f, _)| f.clone()).collect();

    let actual_fs_idents: Vec<_> = actual_fs.iter().map(|f| f.ident.clone().unwrap()).collect();
    let synthetic_fs_idents: Vec<_> = synthetic_fs.iter().map(|f| f.ident.clone().unwrap()).collect();
    let synthetic_fs_tokens: Vec<_> = synthetic_fs_with_tokens.iter().map(|(_, ts)| ts.clone()).collect();

    let serde_prefix = syn::Path::parse.parse2(quote!{ serde }).unwrap();
    let struct_attrs_internal = { 
        let first_serde_idx = match struct_attrs.iter().enumerate().find(|(_idx, attr)| attr.path == serde_prefix) {
            Some((idx, _)) => idx,
            None => struct_attrs.len(),
        };
        let mut attrs = struct_attrs.clone();
        attrs.insert(first_serde_idx, syn::parse_quote!{#[derive(Deserialize)]});
        attrs
    };

    let derive_prefix = syn::Path::parse.parse2(quote!{ derive }).unwrap();
    let serde_prefixes: Vec<_> = vec![quote!{serde}, quote!{serde_as}, quote!{serde_with::serde_as}].into_iter().map(|ts| syn::Path::parse.parse2(ts).unwrap()).collect();
    let mut derive_serialize_flag = false;
    for attr in struct_attrs {
        if attr.path == derive_prefix {
            let meta: Meta = attr.parse_meta().unwrap();
            match meta {
                Meta::List(ml) => {
                    derive_serialize_flag |= ml.nested.iter().any(|nm| *nm == NestedMeta::Meta(Meta::Path(syn::Path::parse.parse2(quote!{Serialize}).unwrap())))
                },
                _ => {
                    panic!("unsupported meta for derive");
                }
            }
        }
    }
    let struct_attrs_external = if derive_serialize_flag {
        // ok, there is #[derive(Serialize)]
        struct_attrs.clone()
    } else {
        // remove all serde, serde_with and serde_as entries
        struct_attrs.iter().cloned()
            .filter(|attr| !serde_prefixes.contains(&attr.path))
            .collect()
    };
    let actual_fs_external = if derive_serialize_flag {
        actual_fs.clone()
    } else {
        actual_fs.iter().cloned()
            .map(|mut f| {
                f.attrs = f.attrs.into_iter().filter(|attr| !serde_prefixes.contains(&attr.path)).collect();
                f
            })
            .collect()
    };

    let output = quote! {

        #(#struct_attrs_external)*
        #struct_vis struct #struct_ident #struct_generics {
            #(#actual_fs_external,)*
            #(#synthetic_fs,)*
        }

        impl<'de> Deserialize<'de> for #struct_ident #struct_generics {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>
            {
                #(#struct_attrs_internal)*
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
