#![no_std]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Item, ImplItem, TraitItem};

#[proc_macro_attribute]
pub fn unconst(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse::<Item>(item).unwrap() {
        Item::Fn(mut r#fn) => {
            r#fn.sig.constness = None;
            return quote!(#r#fn).into();
        }
        Item::Impl(mut r#impl) => {
            for item in r#impl.items.iter_mut() {
                match item {
                    ImplItem::Method(method) => method.sig.constness = None,
                    _ => continue
                };
            }
            return quote!(#r#impl).into()
        }
        Item::Trait(mut r#trait) => {
            for item in r#trait.items.iter_mut() {
                match item {
                    TraitItem::Method(method) => method.sig.constness = None,
                    _ => continue
                };
            }
            return quote!(#r#trait).into()
        }
        Item::Verbatim(_tt) => unimplemented!(),
        _ => panic!("Input is neither a function, a trait nor an impl")
    }
}
