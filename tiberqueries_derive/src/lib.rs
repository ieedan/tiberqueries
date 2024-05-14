use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Lit, Meta, MetaNameValue};
use syn::{DeriveInput, GenericArgument, PathArguments, Type};

#[proc_macro_derive(FromRow, attributes(sql_name))]
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
        syn::Data::Enum(_) => {
            return Err(syn::Error::new(input.span(), "Enums cannot derive FromRow"))
        }
        syn::Data::Union(_) => {
            return Err(syn::Error::new(
                input.span(),
                "Unions cannot derive FromRow",
            ))
        }
    };

    let mut field_strings: Vec<proc_macro2::TokenStream> = vec![];

    for f in data.fields {
        let name = f.ident.clone().unwrap().to_string();
        let mut sql_name = f.ident.as_ref().unwrap().to_string();
        let option_type = is_option_type(&f.ty);
        for attr in &f.attrs {
            if let Ok(Meta::NameValue(MetaNameValue { path, lit, .. })) = attr.parse_meta() {
                if path.is_ident("sql_name") {
                    if let Lit::Str(lit_str) = lit {
                        sql_name = lit_str.value();
                    }
                }
            }
        }

        if option_type {
            let gen = quote! {
                #name: row.get(stringify!(#sql_name)),
            };
            field_strings.push(gen.into());
        } else {
            let gen = quote! {
                #name: row.get(stringify!(#sql_name)).unwrap(),
            };
            field_strings.push(gen.into());
        }
    }

    let gen = quote! {
        impl FromRow for #name {
            fn from_row(row: tiberius::Row) -> Result<Self, Box<dyn Error>> {
                Ok(Self {
                    #(#field_strings)*
                })
            }
        }
    };

    Ok(gen.into())
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
            let segment = &type_path.path.segments[0];
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments {
                    if angle_bracketed.args.len() == 1 {
                        if let GenericArgument::Type(_) = &angle_bracketed.args[0] {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
