use proc_macro2::{TokenStream, Span};
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;

use crate::definition::{Struct, DecodeMode, NewType, ExtraKind, ChildMode};

pub(crate) struct Common<'a> {
    pub object: &'a Struct,
    pub ctx: &'a syn::Ident,
    #[allow(unused)]
    pub span_type: &'a TokenStream,
}

pub fn emit_struct(s: &Struct, named: bool) -> syn::Result<TokenStream> {
    let s_name = &s.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());
    let children = syn::Ident::new("children", Span::mixed_site());

    let (_, type_gen, _) = s.generics.split_for_impl();
    let mut common_generics = s.generics.clone();
    let span_ty;
    if let Some(ty) = s.trait_props.span_type.as_ref() {
        span_ty = quote!(#ty);
    } else {
        if common_generics.params.is_empty() {
            common_generics.lt_token = Some(Default::default());
            common_generics.gt_token = Some(Default::default());
        }
        common_generics.params.push(syn::parse2(quote!(S)).unwrap());
        span_ty = quote!(S);
        common_generics.make_where_clause().predicates.push(
            syn::parse2(quote!(S: ::kfl::traits::ErrorSpan)).unwrap());
    };
    let trait_gen = quote!(<#span_ty>);
    let (impl_gen, _, bounds) = common_generics.split_for_impl();

    let common = Common {
        object: s,
        ctx: &ctx,
        span_type: &span_ty,
    };

    let check_type = check_type(&common, &node)?;
    let decode_specials = decode_specials(&common, &node)?;
    let decode_args = decode_args(&common, &node)?;
    let decode_props = decode_props(&common, &node)?;
    let decode_children_normal = decode_children(
        &common, &children, Some(quote!(#node.span())))?;
    let assign_extra = assign_extra(&common)?;

    let all_fields = s.all_fields();
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
        quote! { #s_name(#(#assignments),*) }
    };
    let mut extra_traits = Vec::new();
    let partial_compatible = s.spans.is_empty() &&
        !s.has_arguments &&
        !s.has_properties &&
        s.children.iter().all(|child| child.default.is_some());
    if partial_compatible {
        let node = syn::Ident::new("node", Span::mixed_site());
        // let name = syn::Ident::new("name", Span::mixed_site());
        // let value = syn::Ident::new("value", Span::mixed_site());
        let decode_partial = decode_partial(&common, &node)?;
        // let insert_property = insert_property(&common, &name, &value)?;
        extra_traits.push(quote! {
            impl #impl_gen ::kfl::traits::DecodePartial #trait_gen
                for #s_name #type_gen
                #bounds
            {
                fn decode_partial(&mut self,
                    #node: &::kfl::ast::SpannedNode<#span_ty>,
                    #ctx: &mut ::kfl::decode::Context<#span_ty>)
                    -> Result<bool, ::kfl::errors::DecodeError<#span_ty>>
                {
                    #decode_partial
                }
                // fn insert_property(&mut self,
                //     #name: &::kfl::span::Spanned<Box<str>, #span_ty>,
                //     #value: &::kfl::ast::Value<#span_ty>,
                //     #ctx: &mut ::kfl::decode::Context<#span_ty>)
                //     -> Result<bool, ::kfl::errors::DecodeError<#span_ty>>
                // {
                //     #insert_property
                // }
            }
        });
    }
    if !s.has_arguments && !s.has_properties && s.spans.is_empty() {
        let decode_children = decode_children(&common, &children, None)?;
        extra_traits.push(quote! {
            impl #impl_gen ::kfl::traits::DecodeChildren #trait_gen
                for #s_name #type_gen
                #bounds
            {
                fn decode_children(
                    #children: &[::kfl::ast::SpannedNode<#span_ty>],
                    #ctx: &mut ::kfl::decode::Context<#span_ty>)
                    -> Result<Self, ::kfl::errors::DecodeError<#span_ty>>
                {
                    #decode_children
                    #assign_extra
                    Ok(#struct_val)
                }
            }
        });
    }
    Ok(quote! {
        #(#extra_traits)*
        impl #impl_gen ::kfl::traits::Decode #trait_gen for #s_name #type_gen
            #bounds
        {
            fn decode_node(#node: &::kfl::ast::SpannedNode<#span_ty>,
                           #ctx: &mut ::kfl::decode::Context<#span_ty>)
                -> Result<Self, ::kfl::errors::DecodeError<#span_ty>>
            {
                #check_type
                #decode_specials
                #decode_args
                #decode_props
                let #children = #node.children.as_ref()
                    .map(|lst| &lst[..]).unwrap_or(&[]);
                #decode_children_normal
                #assign_extra
                Ok(#struct_val)
            }
        }
    })
}

pub fn emit_new_type(s: &NewType) -> syn::Result<TokenStream> {
    let s_name = &s.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());
    Ok(quote! {
        impl<S: ::kfl::traits::ErrorSpan>
            ::kfl::traits::Decode<S> for #s_name
        {
            fn decode_node(#node: &::kfl::ast::SpannedNode<S>,
                           #ctx: &mut ::kfl::decode::Context<S>)
                -> Result<Self, ::kfl::errors::DecodeError<S>>
            {
                if #node.arguments.len() > 0 ||
                    #node.properties.len() > 0 ||
                    #node.children.is_some()
                {
                    ::kfl::traits::Decode::decode_node(#node, #ctx)
                        .map(Some)
                        .map(#s_name)
                } else {
                    Ok(#s_name(None))
                }
            }
        }
    })
}

pub(crate) fn decode_enum_item(s: &Common,
    s_name: impl ToTokens, node: &syn::Ident, named: bool)
    -> syn::Result<TokenStream>
{
    let children = syn::Ident::new("children", Span::mixed_site());
    let decode_args = decode_args(s, node)?;
    let decode_props = decode_props(s, node)?;
    let decode_children = decode_children(s, &children,
                                          Some(quote!(#node.span())))?;
    let assign_extra = assign_extra(s)?;
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
        quote! { #s_name(#(#assignments),*) }
    };
    Ok(quote! {
        #decode_args
        #decode_props
        let #children = #node.children.as_ref()
            .map(|lst| &lst[..]).unwrap_or(&[]);
        #decode_children
        #assign_extra
        Ok(#struct_val)
    })
}

fn decode_value(val: &syn::Ident, ctx: &syn::Ident, mode: &DecodeMode,
                ty: &syn::Type)
    -> syn::Result<TokenStream>
{
    match mode {
        DecodeMode::Normal => {
            Ok(quote! {
                <#ty as ::kfl::traits::DecodeScalar<_>>::decode(#val, #ctx)
            })
        }
        // DecodeMode::Str if optional => {
        //     Ok(quote![{
        //         if let Some(typ) = &#val.type_name {
        //             #ctx.emit_error(::kfl::errors::DecodeError::TypeName {
        //                 span: typ.span().clone(),
        //                 found: Some((**typ).clone()),
        //                 expected: ::kfl::errors::ExpectedType::no_type(),
        //                 rust_type: "str", // TODO(tailhook) show field type
        //             });
        //         }
        //         match *#val.literal {
        //             ::kfl::ast::Literal::String(ref s) => {
        //                 ::std::str::FromStr::from_str(s).map_err(|e| {
        //                     ::kfl::errors::DecodeError::conversion(
        //                         &#val.literal, e)
        //                 })
        //                 .map(Some)
        //             }
        //             ::kfl::ast::Literal::Null => Ok(None),
        //             _ => {
        //                 #ctx.emit_error(
        //                     ::kfl::errors::DecodeError::scalar_kind(
        //                         ::kfl::decode::Kind::String,
        //                         &#val.literal,
        //                     )
        //                 );
        //                 Ok(None)
        //             }
        //         }
        //     }])
        // }
        DecodeMode::Str => {
            Ok(quote![{
                if let Some(typ) = &#val.type_name {
                    #ctx.emit_error(::kfl::errors::DecodeError::TypeName {
                        span: typ.span().clone(),
                        found: Some((**typ).clone()),
                        expected: ::kfl::errors::ExpectedType::no_type(),
                        rust_type: "str", // TODO(tailhook) show field type
                    });
                }
                match *#val.literal {
                    ::kfl::ast::Literal::String(ref s) => {
                        ::std::str::FromStr::from_str(s).map_err(|e| {
                            ::kfl::errors::DecodeError::conversion(
                                &#val.literal, e)
                        })
                    }
                    _ => Err(::kfl::errors::DecodeError::scalar_kind(
                        ::kfl::decode::Kind::String,
                        &#val.literal,
                    )),
                }
            }])
        }
        // DecodeMode::Bytes if optional => {
        //     Ok(quote! {
        //         if matches!(&*#val.literal, ::kfl::ast::Literal::Null) {
        //             Ok(None)
        //         } else {
        //             match ::kfl::decode::bytes(#val, #ctx).try_into() {
        //                 Ok(v) => Ok(Some(v)),
        //                 Err(e) => {
        //                     #ctx.emit_error(
        //                         ::kfl::errors::DecodeError::conversion(
        //                             &#val.literal, e));
        //                     Ok(None)
        //                 }
        //             }
        //         }
        //     })
        // }
        DecodeMode::Bytes => {
            Ok(quote! {
                ::kfl::decode::bytes(#val, #ctx).try_into()
                .map_err(|e| ::kfl::errors::DecodeError::conversion(
                        &#val.literal, e))
            })
        }
    }
}

fn check_type(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let name = heck::ToKebabCase::to_kebab_case(
        &s.object.ident.unraw().to_string()[..]);
    Ok(quote! {
        if #node.node_name.as_ref() != #name {
            return Err(::kfl::errors::DecodeError::unexpected(
                #node, "node", format!("unexpected node `{}`",
                #node.node_name.as_ref())
            ))
        }
    })
}

fn decode_specials(s: &Common, node: &syn::Ident)
    -> syn::Result<TokenStream>
{
    let ctx = s.ctx;
    let spans = s.object.spans.iter().flat_map(|span| {
        let fld = &span.field.tmp_name;
        quote! {
            let #fld = ::kfl::traits::DecodeSpan::decode_span(
                #node.span(),
                #ctx,
            );
        }
    });
    // let type_names = s.object.type_names.iter().flat_map(|type_name| {
    //     let fld = &type_name.field.tmp_name;
    //     quote! {
    //         let #fld = if let Some(tn) = #node.type_name.as_ref() {
    //             tn.as_str()
    //                 .parse()
    //                 .map_err(|e| {
    //                     ::kfl::errors::DecodeError::conversion(tn, e)
    //                 })?
    //         } else {
    //             return Err(::kfl::errors::DecodeError::missing(
    //                 #node, "type name required"));
    //         };
    //     }
    // });
    // let validate_type = if s.object.type_names.is_empty() {
    //     Some(quote! {
    //         if let Some(type_name) = &#node.type_name {
    //             #ctx.emit_error(::kfl::errors::DecodeError::unexpected(
    //                         type_name, "type name",
    //                         "no type name expected for this node"));
    //         }
    //     })
    // } else {
    //     None
    // };
    Ok(quote! {
        #(#spans)*
    })
}

fn decode_args(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let mut decoder = Vec::new();
    let iter_args = syn::Ident::new("iter_args", Span::mixed_site());
    decoder.push(quote! {
        let mut #iter_args = #node.arguments.iter();
    });
    for arg in &s.object.arguments {
        let fld = &arg.field.tmp_name;
        let val = syn::Ident::new("val", Span::mixed_site());
        let decode_value = decode_value(&val, ctx, &arg.decode, &arg.field.ty)?;
        match &arg.default {
            None => {
                let error = if arg.field.is_indexed() {
                    "additional argument is required".into()
                } else {
                    format!("additional argument `{}` is required", fld.unraw())
                };
                decoder.push(quote! {
                    let #val =
                        #iter_args.next().ok_or_else(|| {
                            ::kfl::errors::DecodeError::missing(
                                #node, #error)
                        })?;
                    let #fld = #decode_value?;
                });
            }
            Some(default_value) => {
                let default = if let Some(expr) = default_value {
                    quote!(#expr)
                } else {
                    quote!(::std::default::Default::default())
                };
                decoder.push(quote! {
                    let #fld = #iter_args.next().map(|#val| {
                        #decode_value
                    }).transpose()?.unwrap_or_else(|| {
                        #default
                    });
                });
            }
        }
    }
    if let Some(var_args) = &s.object.var_args {
        let fld = &var_args.field.tmp_name;
        let val = syn::Ident::new("val", Span::mixed_site());
        let decode_value = decode_value(&val, ctx, &var_args.decode,
                                        &var_args.field.ty)?;
        decoder.push(quote! {
            let #fld = #iter_args.map(|#val| {
                #decode_value
            }).collect::<Result<_, _>>()?;
        });
    } else {
        decoder.push(quote! {
            if let Some(val) = #iter_args.next() {
                return Err(::kfl::errors::DecodeError::unexpected(
                        &val.literal, "argument",
                        "unexpected argument"));
            }
        });
    }
    Ok(quote! { #(#decoder)* })
}

fn decode_props(s: &Common, node: &syn::Ident)
    -> syn::Result<TokenStream>
{
    let mut declare_empty = Vec::new();
    let mut match_branches = Vec::new();
    let mut postprocess = Vec::new();

    let ctx = s.ctx;
    let val = syn::Ident::new("val", Span::mixed_site());
    let name = syn::Ident::new("name", Span::mixed_site());
    let name_str = syn::Ident::new("name_str", Span::mixed_site());

    for prop in &s.object.properties {
        let fld = &prop.field.tmp_name;
        let prop_name = &prop.name;
        let seen_name = format_ident!("seen_{}", fld, span = Span::mixed_site());
        if false /* TODO prop.flatten */ {
            declare_empty.push(quote! {
                let mut #fld = ::std::default::Default::default();
            });
            match_branches.push(quote! {
                _ if ::kfl::traits::DecodePartial::
                    insert_property(&mut #fld, #name, #val, #ctx)?
                => {}
            });
        } else {
            let decode_value = decode_value(&val, ctx, &prop.decode,
                                            &prop.field.ty)?;
            declare_empty.push(quote! {
                let mut #fld = None;
                let mut #seen_name = false;
            });
            match_branches.push(quote! {
                #prop_name => {
                    #fld = Some(#decode_value?);
                }
            });
            let req_msg = format!("property `{}` is required", prop_name);
            if let Some(value) = &prop.default {
                let default = if let Some(expr) = value {
                    quote!(#expr)
                } else {
                    quote!(::std::default::Default::default())
                };
                postprocess.push(quote! {
                    let #fld = #fld.unwrap_or_else(|| #default);
                });
            } else {
                postprocess.push(quote! {
                    let #fld = #fld.ok_or_else(|| {
                        ::kfl::errors::DecodeError::missing(
                            #node, #req_msg)
                    })?;
                });
            }
        }
    }
    if let Some(var_props) = &s.object.var_props {
        let fld = &var_props.field.tmp_name;
        let decode_value = decode_value(&val, ctx, &var_props.decode,
                                        &var_props.field.ty)?;
        declare_empty.push(quote! {
            let mut #fld = Vec::new();
        });
        match_branches.push(quote! {
            #name_str => {
                let converted_name = #name_str.parse()
                    .map_err(|e| {
                        ::kfl::errors::DecodeError::conversion(#name, e)
                    })?;
                #fld.push((
                    converted_name,
                    #decode_value?,
                ));
            }
        });
        postprocess.push(quote! {
            let #fld = #fld.into_iter().collect();
        });
    } else {
        match_branches.push(quote! {
            #name_str => {
                return Err(::kfl::errors::DecodeError::unexpected(
                    #name, "property",
                    format!("unexpected property `{}`",
                            #name_str.escape_default())));
            }
        });
    };
    Ok(quote! {
        #(#declare_empty)*
        for (#name, #val) in #node.properties.iter() {
            match &***#name {
                #(#match_branches)*
            }
        }
        #(#postprocess)*
    })
}

// fn unwrap_fn(parent: &Common,
//              func: &syn::Ident, name: &syn::Ident, ty: &syn::Type, attrs: &FieldAttrs)
//     -> syn::Result<TokenStream>
// {
//     let ctx = parent.ctx;
//     let span_ty = parent.span_type;
//     let mut bld = StructBuilder::new(
//         format_ident!("Wrap_{}", name, span = Span::mixed_site()),
//         parent.object.trait_props.clone(),
//         parent.object.generics.clone(),
//     );
//     bld.add_field(Field::new_named(name, ty), attrs)?;
//     let object = bld.build();
//     let common = Common {
//         object: &object,
//         ctx: parent.ctx,
//         span_type: parent.span_type,
//     };

//     let node = syn::Ident::new("node", Span::mixed_site());
//     let children = syn::Ident::new("children", Span::mixed_site());
//     let decode_args = decode_args(&common, &node)?;
//     let decode_props = decode_props(&common, &node)?;
//     let decode_children = decode_children(&common, &children,
//                                           Some(quote!(#node.span())))?;
//     Ok(quote! {
//         let mut #func = |#node: &::kfl::ast::SpannedNode<#span_ty>,
//                          #ctx: &mut ::kfl::decode::Context<#span_ty>|
//         {
//             #decode_args
//             #decode_props
//             let #children = #node.children.as_ref()
//                 .map(|lst| &lst[..]).unwrap_or(&[]);
//             #decode_children

//             Ok(#name)
//         };
//     })
// }

fn decode_partial(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let mut branches = vec![quote! {
        if false {
            Ok(false)
        }
    }];
    for child_def in &s.object.children {
        let dest = &child_def.field.from_self();
        let ty = &child_def.field.ty;
        match &child_def.mode {
            ChildMode::Normal => {
                let dup_err = quote! {
                    format!("duplicate node `{}`, single node expected",
                            #node.node_name.as_ref())
                };
                branches.push(quote! {
                    else if let Ok(value) = <#ty as ::kfl::traits::Decode<_>>
                        ::decode_node(#node, #ctx)
                    {
                        if #dest.is_some() {
                            Err(
                                ::kfl::errors::DecodeError::unexpected(
                                &#node.node_name, "node", #dup_err))
                        } else {
                            #dest = value;
                            Ok(true)
                        }
                    }
                });
            }
            ChildMode::Multi => {
                branches.push(quote! {
                    else if let Ok(true) = <#ty as ::kfl::traits::DecodePartial<_>>
                        ::decode_partial(&mut #dest, #node, #ctx)
                    {
                        Ok(true)
                    }
                });
            }
            ChildMode::Flatten => {
                branches.push(quote! {
                    else if <#ty as ::kfl::traits::DecodePartial<_>>
                        ::decode_partial(&mut #dest, #node, #ctx)?
                    {
                        Ok(true)
                    }
                });
            }
        }
    }
    branches.push(quote! {
        else {
            Ok(false)
        }
    });
    Ok(quote!(#(#branches)*))
}

// fn insert_property(s: &Common, name: &syn::Ident, value: &syn::Ident)
//     -> syn::Result<TokenStream>
// {
//     let ctx = s.ctx;
//     let mut match_branches = Vec::with_capacity(s.object.children.len());
//     for prop in &s.object.properties {
//         let dest = &prop.field.from_self();
//         let prop_name = &prop.name;
//         if false /* TODO prop.flatten */ {
//             match_branches.push(quote! {
//                 _ if ::kfl::traits::DecodePartial
//                     ::insert_property(&mut #dest, #name, #value, #ctx)?
//                 => Ok(true),
//             });
//         } else {
//             let decode_value = decode_value(&value, ctx, &prop.decode,
//                                             &prop.field.ty)?;
//             match_branches.push(quote! {
//                 #prop_name => {
//                     #dest = Some(#decode_value?);
//                     Ok(true)
//                 }
//             });
//         }
//     }
//     Ok(quote! {
//         match &***#name {
//             #(#match_branches)*
//             _ => Ok(false),
//         }
//     })
// }

fn decode_children(s: &Common, children: &syn::Ident,
                   err_span: Option<TokenStream>)
    -> syn::Result<TokenStream>
{
    let mut declare_empty = Vec::new();
    let mut branches = vec![quote! {
        if false {
            None
        }
    }];
    let mut postprocess = Vec::new();

    let ctx = s.ctx;
    let child = syn::Ident::new("child", Span::mixed_site());
    for child_def in &s.object.children {
        let fld = &child_def.field.tmp_name;
        let child_name = &s.object.ident.to_string();
        let ty = &child_def.field.ty;
        match child_def.mode {
            ChildMode::Flatten => {
                declare_empty.push(quote! {
                    let mut #fld = ::std::default::Default::default();
                });
                branches.push(quote! {
                    else if let Ok(true) = <#ty as ::kfl::traits::DecodePartial<_>>
                        ::decode_partial(&mut #fld, #child, #ctx) {
                        None
                    }
                });
            }
            ChildMode::Multi => {
                declare_empty.push(quote! {
                    let mut #fld = Vec::new();
                });
                let ctx = &s.ctx;
                branches.push(quote! {
                    else if let Ok(true) = <#ty as ::kfl::traits::DecodePartial<_>>
                        ::decode_partial(&mut #fld, #child, #ctx)
                    {
                        None
                    }
                });
                if let Some(default_value) = &child_def.default {
                    let default = if let Some(expr) = default_value {
                        quote!(#expr)
                    } else {
                        quote!(::std::default::Default::default())
                    };
                    postprocess.push(quote! {
                        let #fld = if #fld.is_empty() {
                            #default
                        } else {
                            #fld.into_iter().collect()
                        };
                    });
                } else {
                    postprocess.push(quote! {
                        let #fld = #fld.into_iter().collect();
                    });
                }
            }
            ChildMode::Normal => {
                declare_empty.push(quote! {
                    let mut #fld = None;
                });
                let dup_err = quote! {
                    format!("duplicate node `{}`, single node expected",
                            #child.node_name.as_ref())
                };
                branches.push(quote! {
                    else if let Ok(value) = <#ty as ::kfl::traits::Decode<_>>
                        ::decode_node(#child, #ctx)
                    {
                        if #fld.is_some() {
                            Some(Err(
                                ::kfl::errors::DecodeError::unexpected(
                                &#child.node_name, "node", #dup_err)))
                        } else {
                            #fld = Some(value);
                            None
                        }
                    }
                });
                let req_msg = format!("child node for field `{}` is required",
                                      child_name);
                if let Some(default_value) = &child_def.default {
                    let default = if let Some(expr) = default_value {
                        quote!(#expr)
                    } else {
                        quote!(::std::default::Default::default())
                    };
                    postprocess.push(quote! {
                        let #fld = #fld.unwrap_or_else(|| #default);
                    });
                } else {
                    if let Some(span) = &err_span {
                        postprocess.push(quote! {
                            let #fld = #fld.ok_or_else(|| {
                                ::kfl::errors::DecodeError::Missing {
                                    span: #span.clone(),
                                    message: #req_msg.into(),
                                }
                            })?;
                        });
                    } else {
                        postprocess.push(quote! {
                            let #fld = #fld.ok_or_else(|| {
                                ::kfl::errors::DecodeError::MissingNode {
                                    message: #req_msg.into(),
                                }
                            })?;
                        });
                    }
                }
            }
        }
    }
    // TODO(rnarkk) return Err?
    branches.push(quote! {
        else {
            #ctx.emit_error(::kfl::errors::DecodeError::unexpected(
                #child, "node",
                format!("unexpected node `{}`",
                        #child.node_name.as_ref())));
            None
        }
    });
    Ok(quote! {
        #(#declare_empty)*
        #children.iter().flat_map(|#child| {
            #(#branches)*
        }).collect::<Result<(), ::kfl::errors::DecodeError<_>>>()?;
        #(#postprocess)*
    })
}

fn assign_extra(s: &Common) -> syn::Result<TokenStream> {
    let items = s.object.extra_fields.iter().map(|fld| {
        match fld.kind {
            ExtraKind::Auto => {
                let name = &fld.field.tmp_name;
                quote!(let #name = ::std::default::Default::default();)
            }
        }
    });
    Ok(quote!(#(#items)*))
}
