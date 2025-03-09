use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, LitInt, MetaList};

#[proc_macro_attribute]
pub fn register(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as DeriveInput);
    let name = item.ident.clone();

    let mut address: Option<LitInt> = None;
    let mut reset: Option<LitInt> = None;

    let args_parser = syn::meta::parser(|arg| {
        if arg.path.is_ident("addr") {
            address = Some(arg.value()?.parse()?);
            Ok(())
        } else if arg.path.is_ident("reset") {
            reset = Some(arg.value()?.parse()?);
            Ok(())
        } else {
            Err(arg.error("Unknown property"))
        }
    });

    syn::parse_macro_input!(args with args_parser);

    let out = quote! {
        #[::bitbybit::bitfield(u8, default=#reset)]
        #item

        impl ::core::convert::From<u8> for #name {
            fn from(v: u8) -> Self {
                Self::new_with_raw_value(v)
            }
        }

        impl ::core::convert::From<#name> for u8 {
            fn from(v: #name) -> u8 {
                v.raw_value()
            }
        }

        impl crate::peripheral::Register for #name {
            const ADDRESS: u32 = #address;
        }
    };

    out.into()
}
