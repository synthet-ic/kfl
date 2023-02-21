use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::ext::IdentExt;

use crate::{
    definition::{Enum, VariantKind},
    node
};

pub(crate) struct Common<'a> {
    pub object: &'a Enum,
    pub ctx: &'a syn::Ident,
}

pub fn emit_enum(e: &Enum) -> syn::Result<TokenStream> {
    let name = &e.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());

    let (_, type_gen, _) = e.generics.split_for_impl();
    let common_generics = e.generics.clone();
    let (impl_gen, _, bounds) = common_generics.split_for_impl();

    let common = Common {
        object: e,
        ctx: &ctx,
    };
    let check_type = check_type(&common, &node)?;
    let decode = decode(&common, &node)?;
    Ok(quote! {
        impl #impl_gen ::kfl::Decode for #name #type_gen
            #bounds
        {
            fn decode(#node: &::kfl::ast::Node,
                      #ctx: &mut ::kfl::decode::Context)
                -> Result<Self, ::kfl::errors::DecodeError>
            {
                #check_type
                #decode
            }
        }
    })
}

fn check_type(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let name = heck::ToKebabCase::to_kebab_case(
        &s.object.ident.unraw().to_string()[..]);
    Ok(quote! {
        if let Some(type_name) = #node.type_name.as_ref() {
            let type_name = type_name.as_ref();
            if type_name != #name {
                return Err(::kfl::errors::DecodeError::unexpected(
                    #ctx.span(&#node), "node", format!("unexpected node `({}){}`",
                    type_name,
                    #node.node_name.as_ref())
                ))
            }
        }
    })
}

fn decode(e: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = e.ctx;
    let mut branches = Vec::with_capacity(e.object.variants.len());
    let enum_name = &e.object.ident;
    for var in &e.object.variants {
        let name = &var.name;
        let variant_name = &var.ident;
        match &var.kind {
            VariantKind::Unit => {
                branches.push(quote! {
                    #name => {
                        for arg in &#node.arguments {
                            return Err(
                                ::kfl::errors::DecodeError::unexpected(
                                    #ctx.span(&arg.literal), "argument",
                                    "unexpected argument"));
                        }
                        for (name, _) in &#node.properties {
                            return Err(
                                ::kfl::errors::DecodeError::unexpected(
                                    #ctx.span(&name), "property",
                                    format!("unexpected property `{}`",
                                            name.escape_default())));
                        }
                        if let Some(children) = &#node.children {
                            for child in children.iter() {
                                return Err(
                                    ::kfl::errors::DecodeError::unexpected(
                                        #ctx.span(&child), "node",
                                        format!("unexpected node `{}`",
                                            child.node_name.escape_default())
                                    ));
                            }
                        }
                        Ok(#enum_name::#variant_name)
                    }
                });
            }
            VariantKind::Nested { ty } => {
                branches.push(quote! {
                    #name => <#ty as ::kfl::Decode>::decode(#node, #ctx)
                        .map(#enum_name::#variant_name),
                });
            }
            VariantKind::Tuple(s) => {
                let common = node::Common {
                    object: s,
                    ctx,
                };
                let decode = node::decode_variant(
                    &common,
                    quote!(#enum_name::#variant_name),
                    node,
                    false,
                )?;
                branches.push(quote! {
                    #name => { #decode }
                });
            }
            VariantKind::Named(s) => {
                let common = node::Common {
                    object: s,
                    ctx,
                };
                let decode = node::decode_variant(
                    &common,
                    quote!(#enum_name::#variant_name),
                    node,
                    true,
                )?;
                branches.push(quote! {
                    #name => { #decode }
                });
            },
        }
    }
    // TODO(tailhook) use strsim to find similar names
    let err = if e.object.variants.len() <= 3 {
        format!("expected one of {}",
                e.object.variants.iter()
                .map(|v| format!("`{}`", v.name.escape_default()))
                .collect::<Vec<_>>()
                .join(", "))
    } else {
        format!("expected `{}`, `{}`, or one of {} others",
                e.object.variants[0].name.escape_default(),
                e.object.variants[1].name.escape_default(),
                e.object.variants.len() - 2)
    };
    Ok(quote! {
        match &*#node.node_name {
            #(#branches)*
            name_str => {
                Err(::kfl::errors::DecodeError::conversion(
                        #ctx.span(&#node.node_name), #err))
            }
        }
    })
}
