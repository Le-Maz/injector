extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Path};

#[proc_macro_derive(Injectable, attributes(inject))]
pub fn injectable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(data_struct) = input.data else {
        return quote!(compile_error!("This macro should only be used on structs")).into();
    };

    let name = input.ident;

    let mut fields = Vec::with_capacity(data_struct.fields.len());
    for field in data_struct.fields {
        let ident = field.ident;
        let ty = field.ty;
        if let Some(ident) = ident {
            if let Some(attr) = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("inject"))
            {
                let Ok(constructor) = attr.parse_args::<Path>() else {
                    return quote!(compile_error!("Bad argument for attribute")).into();
                };
                fields.extend(quote!(#ident: #constructor(),));
            } else {
                fields.extend(quote!(#ident: <#ty>::construct(injector),));
            }
        } else {
            fields.extend(quote!(#ty,));
        }
    }

    quote! {
        impl ::injector::Injectable for #name {
            fn construct(injector: &mut ::injector::Injector) -> Self {
                Self {
                    #(#fields)*
                }
            }
        }
    }
    .into()
}
