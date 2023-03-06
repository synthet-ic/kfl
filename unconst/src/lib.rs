// #![no_std]

use core::clone::Clone;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, ItemFn, ItemTrait, TraitItem, ItemImpl, ImplItem};

#[proc_macro_attribute]
pub fn unconst(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse::<ItemFn>(item.clone()) {
        Ok(mut r#fn) => {
            r#fn.sig.constness = None;
            return quote!(#r#fn).into();
        }
        Err(_) => {}
    };
    match parse::<ItemTrait>(item.clone()) {
        Ok(mut r#trait) => {
            for item in r#trait.items.iter_mut() {
                match item {
                    TraitItem::Method(method) => method.sig.constness = None,
                    _ => continue
                };
            }
            return quote!(#r#trait).into()
        }
        Err(_) => {}
    };
    match parse::<ItemImpl>(item.clone()) {
        Ok(mut r#impl) => {
            for item in r#impl.items.iter_mut() {
                match item {
                    ImplItem::Method(method) => method.sig.constness = None,
                    _ => continue
                };
            }
            return quote!(#r#impl).into()
        }
        Err(_) => {}
    };
    panic!("Input is neither a function, a trait nor an impl");
}
