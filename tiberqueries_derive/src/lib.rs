use heck::AsUpperCamelCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, GenericArgument, Lit, Meta, MetaNameValue,
    PathArguments, TypePath,
};
use syn::{DeriveInput, Type};

#[proc_macro_derive(FromRow, attributes(sql_name, to_pascal))]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_from_row(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_derive_from_row(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "FromRow can only be derived from Structs",
            ))
        }
    };

    let mut to_pascal = false;
    for attr in &input.attrs {
        if let Ok(Meta::Path(path)) = attr.parse_meta() {
            if path.is_ident("to_pascal") {
                to_pascal = true;
            }
        }
    }

    let mut field_strings: Vec<proc_macro2::TokenStream> = vec![];

    for f in data.fields {
        let name = f.ident.clone().unwrap();
        let mut sql_name = if to_pascal {
            AsUpperCamelCase(f.ident.as_ref().unwrap().to_string()).to_string()
        } else {
            f.ident.as_ref().unwrap().to_string()
        };
        let option_type = is_option_type(&f.ty);
        let is_string = is_string(&f.ty);
        for attr in &f.attrs {
            if let Ok(Meta::NameValue(MetaNameValue { path, lit, .. })) = attr.parse_meta() {
                if path.is_ident("sql_name") {
                    if let Lit::Str(lit_str) = lit {
                        sql_name = lit_str.value();
                    }
                }
            }
        }

        // To make it easier to manage objects all the data should be owned
        // because of this &str must be converted to String
        let gen = if option_type {
            if is_string {
                quote! {
                    #name: string(row.get(#sql_name)),
                }
            } else {
                quote! {
                    #name: row.get(#sql_name),
                }
            }
        } else {
            if is_string {
                quote! {
                    #name: string(row.get(#sql_name)).unwrap(),
                }
            } else {
                quote! {
                    #name: row.get(#sql_name).unwrap(),
                }
            }
        };
        field_strings.push(gen.into());
    }

    let gen = quote! {
        // we put allow dead code in case there aren't any string types
        #[allow(dead_code)]
        fn string(str: Option<&str>) -> Option<String> {
            if let Some(str) = str {
                Some(String::from(str))
            } else {
                None
            }
        }

        impl FromRow for #name {
            fn from_row(row: tiberius::Row) -> Self {
                Self {
                    #(#field_strings)*
                }
            }
        }
    };

    Ok(gen.into())
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        path.segments
            .iter()
            .any(|segment| segment.ident == "Option")
    } else {
        false
    }
}

fn is_string(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            if path.segments.len() == 1 {
                let segment = &path.segments[0];
                if segment.ident == "String" {
                    return true;
                } else if segment.ident == "Option" {
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &segment.arguments
                    {
                        if let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) =
                            args.first()
                        {
                            return path
                                .segments
                                .iter()
                                .any(|segment| segment.ident == "String");
                        }
                    }
                }
                false
            } else {
                false
            }
        }
        _ => false,
    }
}
