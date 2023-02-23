use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::ext::IdentExt;

use crate::{
    definition::{Enum, VariantKind},
    node
};

pub(crate) struct Common<'a> {
    pub object: &'a Enum,
    pub ctx: &'a syn::Ident,
}

pub fn emit_decode_enum(e: &Enum) -> syn::Result<TokenStream> {
    let name = &e.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());

    // TODO(rnarkk) merge
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
                      #ctx: &mut ::kfl::context::Context)
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
    for variant in &e.object.variants {
        let name = &variant.name;
        let variant_name = &variant.ident;
        match &variant.kind {
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
                let decode = decode_variant(
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
                let decode = decode_variant(
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

fn decode_variant(s: &node::Common,
    s_name: impl ToTokens, node: &syn::Ident, named: bool)
    -> syn::Result<TokenStream>
{
    let children = syn::Ident::new("children", Span::mixed_site());
    let decode_arguments = node::decode_arguments(s, node)?;
    let decode_properties = node::decode_properties(s, node)?;
    let decode_children = node::decode_children(s, &children,
                                          Some(quote!(ctx.span(&#node))))?;
    let assign_extra = node::assign_extra(s)?;
    let all_fields = s.object.all_fields();
    let struct_val = if named {
        let assignments = all_fields.iter()
            .map(|f| f.as_assign_pair().unwrap());
        quote!(#s_name { #(#assignments,)* })
    } else {
        let mut fields = all_fields.iter()
            .map(|f| (f.as_index().unwrap(), &f.tmp_name))
            .collect::<Vec<_>>();
        fields.sort_by_key(|(idx, _)| *idx);
        assert_eq!(fields.iter().map(|(idx, _)| *idx).collect::<Vec<_>>(),
                   (0..fields.len()).collect::<Vec<_>>(),
                   "all tuple structure fields should be filled in");
        let assignments = fields.iter().map(|(_, v)| v);
        quote!(#s_name(#(#assignments),*))
    };
    Ok(quote! {
        #decode_arguments
        #decode_properties
        let #children = #node.children.as_ref()
            .map(|lst| &lst[..]).unwrap_or(&[]);
        #decode_children
        #assign_extra
        Ok(#struct_val)
    })
}

pub fn emit_encode_enum(e: &Enum) -> syn::Result<TokenStream> {
    let name = &e.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());

    // TODO(rnarkk) merge
    let (_, type_gen, _) = e.generics.split_for_impl();
    let common_generics = e.generics.clone();
    let (impl_gen, _, bounds) = common_generics.split_for_impl();

    let common = Common {
        object: e,
        ctx: &ctx,
    };
    let declare_variant = declare_variant(&node, &name);
    let encode = encode(&common, &node)?;
    Ok(quote! {
        impl #impl_gen ::kfl::Encode for #name #type_gen
            #bounds
        {
            fn encode(&self,
                      #ctx: &mut ::kfl::context::Context)
                -> Result<::kfl::ast::Node, ::kfl::errors::DecodeError>
            {
                #declare_variant
                #encode
            }
        }
    })
}

fn declare_variant(node: &syn::Ident, name: &syn::Ident) -> TokenStream {
    let name = heck::ToKebabCase::to_kebab_case(name.to_string().as_str());
    // Ok(quote! {
    //     if let Some(type_name) = #node.type_name.as_ref() {
    //         let type_name = type_name.as_ref();
    //         if type_name != #name {
    //             return Err(::kfl::errors::DecodeError::unexpected(
    //                 #ctx.span(&#node), "node", format!("unexpected node `({}){}`",
    //                 type_name,
    //                 #node.node_name.as_ref())
    //             ))
    //         }
    //     }
    // })
    quote! {
        let mut #node = ::kfl::ast::Node::new();
        #node.type_name = Some(#name)
    }
}

fn encode(e: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = e.ctx;
    let mut branches = Vec::with_capacity(e.object.variants.len());
    let enum_name = &e.object.ident;
    for variant in &e.object.variants {
        let name = &variant.name;
        let variant_name = &variant.ident;
        match &variant.kind {
            VariantKind::Unit => {
                branches.push(quote! {
                    #name => {
                        // for arg in &#node.arguments {
                        //     return Err(
                        //         ::kfl::errors::DecodeError::unexpected(
                        //             #ctx.span(&arg.literal), "argument",
                        //             "unexpected argument"));
                        // }
                        // for (name, _) in &#node.properties {
                        //     return Err(
                        //         ::kfl::errors::DecodeError::unexpected(
                        //             #ctx.span(&name), "property",
                        //             format!("unexpected property `{}`",
                        //                     name.escape_default())));
                        // }
                        // if let Some(children) = &#node.children {
                        //     for child in children.iter() {
                        //         return Err(
                        //             ::kfl::errors::DecodeError::unexpected(
                        //                 #ctx.span(&child), "node",
                        //                 format!("unexpected node `{}`",
                        //                     child.node_name.escape_default())
                        //             ));
                        //     }
                        // }
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
                let decode = encode_variant(
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
                let decode = encode_variant(
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

fn encode_variant(s: &node::Common,
    s_name: impl ToTokens, node: &syn::Ident, named: bool)
    -> syn::Result<TokenStream>
{
    let children = syn::Ident::new("children", Span::mixed_site());
    let encode_arguments = node::encode_arguments(s, node)?;
    let encode_properties = node::encode_properties(s, node)?;
    let encode_children = node::encode_children(s, &children,
                                          Some(quote!(ctx.span(&#node))))?;
    let assign_extra = node::assign_extra(s)?;
    let all_fields = s.object.all_fields();
    let struct_val = if named {
        let assignments = all_fields.iter()
            .map(|f| f.as_assign_pair().unwrap());
        quote!(#s_name { #(#assignments,)* })
    } else {
        let mut fields = all_fields.iter()
            .map(|f| (f.as_index().unwrap(), &f.tmp_name))
            .collect::<Vec<_>>();
        fields.sort_by_key(|(idx, _)| *idx);
        assert_eq!(fields.iter().map(|(idx, _)| *idx).collect::<Vec<_>>(),
                   (0..fields.len()).collect::<Vec<_>>(),
                   "all tuple structure fields should be filled in");
        let assignments = fields.iter().map(|(_, v)| v);
        quote!(#s_name(#(#assignments),*))
    };
    Ok(quote! {
        #encode_arguments
        #encode_properties
        let #children = #node.children.as_ref()
            .map(|lst| &lst[..]).unwrap_or(&[]);
        #encode_children
        #assign_extra
        Ok(#struct_val)
    })
}
