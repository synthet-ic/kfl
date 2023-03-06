#![no_std]

use core::clone::Clone;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, ItemFn, ItemTrait, TraitItem};

#[proc_macro_attribute]
pub fn unconst(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse::<ItemFn>(item.clone()) {
        Ok(mut item) => {
            item.sig.constness = None;
            return quote!(item).into()
        }
        Err(_) => {}
    };
    match parse::<ItemTrait>(item) {
        Ok(mut item) => {
            for item in item.items.iter_mut() {
                match item {
                    TraitItem::Method(method) => method.sig.constness = None,
                    _ => continue
                };
            }
            return quote!(item).into()
        }
        Err(_) => {}
    };
    panic!("Input is neither a function nor a trait")
}
