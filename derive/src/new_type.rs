use proc_macro2::{TokenStream, Span};
use quote::quote;

use crate::definition::NewType;

pub fn emit_new_type(s: &NewType) -> syn::Result<TokenStream> {
    let s_name = &s.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());
    Ok(quote! {
        impl ::kfl::traits::Decode for #s_name {
            fn decode(#node: &::kfl::ast::Node,
                      #ctx: &mut ::kfl::context::Context)
                -> Result<Self, ::kfl::errors::DecodeError>
            {
                if #node.arguments.len() > 0 ||
                    #node.properties.len() > 0 ||
                    #node.children.is_some()
                {
                    ::kfl::traits::Decode::decode(#node, #ctx)
                        .map(Some)
                        .map(#s_name)
                } else {
                    Ok(#s_name(None))
                }
            }
        }
    })
}
