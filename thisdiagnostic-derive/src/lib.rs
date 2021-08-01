use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(Diagnostic, attributes(help, label, ask))]
pub fn diagnostics_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_diagnostics_macro(ast)
}

fn impl_diagnostics_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = ast.ident;

    match ast.data {
        Data::Enum(enm) => {
            let variants = enm.variants;

            let label_arms = variants.iter().map(|variant| {
                let id = &variant.ident;

                let labels = variant.attrs.iter().find_map(|a| {
                    if a.path.is_ident("label") {
                        let string: syn::LitStr = a.parse_args().unwrap();
                        Some(string.value())
                    } else {
                        None
                    }
                });

                let should_ask = variant
                    .fields
                    .iter()
                    .any(|field| field.attrs.iter().any(|attr| attr.path.is_ident("ask")));

                match variant.fields {
                    syn::Fields::Unit => labels.map(|l| {
                        quote! {
                            #id => #l.into(),
                        }
                    }),
                    syn::Fields::Named(_) => labels.map(|l| {
                        quote! {
                            #id {..} => #l.into(),
                        }
                    }),
                    syn::Fields::Unnamed(_) => {
                        if should_ask {
                            return Some(quote! {
                                #id(err) => err.label(),
                            });
                        }

                        labels.map(|l| {
                            quote! {
                                #id(..) => #l.into(),
                            }
                        })
                    }
                }
            });

            let help_arms = variants.iter().map(|variant| {
                let id = &variant.ident;

                let helps = variant.attrs.iter().find_map(|a| {
                    if a.path.is_ident("help") {
                        let string: syn::LitStr = a.parse_args().unwrap();
                        Some(string.value())
                    } else {
                        None
                    }
                });

                let should_ask = variant
                    .fields
                    .iter()
                    .any(|field| field.attrs.iter().any(|attr| attr.path.is_ident("ask")));

                match variant.fields {
                    syn::Fields::Unit => helps.map(|a| {
                        quote! {
                            #id => Some(#a.into()),
                        }
                    }),
                    syn::Fields::Named(_) => helps.map(|a| {
                        quote! {
                            #id {..} => Some(#a.into()),
                        }
                    }),
                    syn::Fields::Unnamed(_) => {
                        if should_ask {
                            return Some(quote! {
                                #id(err) => err.help(),
                            });
                        };

                        helps.map(|a| {
                            quote! {
                                #id(..) => Some(#a.into()),
                            }
                        })
                    }
                }
            });

            let gen = quote! {
                impl Diagnostic for #name {
                    fn label(&self) -> String {
                        use #name::*;
                        match self {
                            #(#label_arms)*
                            _ => "crate::label".into()
                        }
                    }

                    fn help(&self) -> Option<String> {
                        use #name::*;
                        match self {
                            #(#help_arms)*
                            _ => None
                        }
                    }
                }
            };

            gen.into()
        }
        Data::Struct(_) => {
            let label = ast
                .attrs
                .iter()
                .find_map(|a| {
                    if a.path.is_ident("label") {
                        let string: syn::LitStr = a.parse_args().unwrap();
                        Some(string.value())
                    } else {
                        None
                    }
                })
                .map_or(
                    quote! {
                        "crate::label".into()
                    },
                    |label| {
                        quote! {
                            #label.into()
                        }
                    },
                );

            let help = ast
                .attrs
                .iter()
                .find_map(|a| {
                    if a.path.is_ident("help") {
                        let string: syn::LitStr = a.parse_args().unwrap();
                        Some(string.value())
                    } else {
                        None
                    }
                })
                .map_or(
                    quote! {
                        None
                    },
                    |val| {
                        quote! {
                            Some(#val.into())
                        }
                    },
                );

            let gen = quote! {
                impl Diagnostic for #name {
                    fn label(&self) -> String {
                        #label
                    }

                    fn help(&self) -> Option<String> {
                        #help
                    }
                }
            };

            gen.into()
        }
        _ => todo!(),
    }
}
