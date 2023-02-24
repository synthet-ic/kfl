use alloc::{
    format,
    string::ToString,
    vec,
    vec::Vec
};

use proc_macro2::{TokenStream, Span};
use quote::{format_ident, quote};
use syn::ext::IdentExt;

use crate::definition::{Struct, ExtraKind, ChildMode};

pub(crate) struct Common<'a> {
    pub object: &'a Struct,
    pub ctx: &'a syn::Ident,
}

pub fn emit_decode_struct(s: &Struct, named: bool, partial: bool)
    -> syn::Result<TokenStream>
{
    let s_name = &s.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());
    let children = syn::Ident::new("children", Span::mixed_site());
    let (impl_gen, type_gen, bounds) = s.generics.split_for_impl();

    let common = Common { object: s, ctx: &ctx };
    let check_type = check_type(&common, &node)?;
    let decode_arguments = decode_arguments(&common, &node)?;
    let decode_properties = decode_properties(&common, &node)?;
    let decode_children = decode_children(
        &common, &children, Some(quote!(#ctx.span(&#node))))?;
    let assign_extra = assign_extra(&common)?;

    let all_fields = s.all_fields();
    let struct_expression = if named {
        let assignments = all_fields.iter()
            .map(|f| f.as_assign_pair().unwrap());
        quote!(#s_name { #(#assignments,)* })
    } else {
        let mut fields = all_fields.iter()
            .map(|f| (f.as_index().unwrap(), &f.tmp_name))
            .collect::<Vec<_>>();
        fields.sort_by_key(|(idx, _)| idx.index);
        // assert_eq!(fields.iter().map(|(idx, _)| *idx).collect::<Vec<_>>(),
        //            (0..fields.len()).collect::<Vec<_>>(),
        //            "all tuple structure fields should be filled in");
        let assignments = fields.iter().map(|(_, v)| v);
        quote!(#s_name(#(#assignments),*))
    };
    let mut extra_traits = Vec::new();
    if partial {
        if has_only_children(&s) {
            let node = syn::Ident::new("node", Span::mixed_site());
            let decode_partial = decode_partial(&common, &node)?;
            // let name = syn::Ident::new("name", Span::mixed_site());
            // let scalar = syn::Ident::new("scalar", Span::mixed_site());
            // let insert_property = insert_property(&common, &name, &scalar)?;
            extra_traits.push(quote! {
                impl #impl_gen ::kfl::traits::DecodePartial
                    for #s_name #type_gen
                    #bounds
                {
                    fn decode_partial(&mut self,
                        #node: &::kfl::ast::Node,
                        #ctx: &mut ::kfl::context::Context)
                        -> Result<bool, ::kfl::errors::DecodeError>
                    {
                        #decode_partial
                    }
                    // fn insert_property(&mut self,
                    //     #name: &Box<str>,
                    //     #scalar: &::kfl::ast::Scalar,
                    //     #ctx: &mut ::kfl::context::Context)
                    //     -> Result<bool, ::kfl::errors::DecodeError>
                    // {
                    //     #insert_property
                    // }
                }
            });
        } else {
            return Err(syn::Error::new(s.ident.span(),
                       "not partial compatible"));
        }
    }
    Ok(quote! {
        #(#extra_traits)*
        impl #impl_gen ::kfl::traits::Decode for #s_name #type_gen #bounds {
            fn decode(#node: &::kfl::ast::Node,
                      #ctx: &mut ::kfl::context::Context)
                -> Result<Self, ::kfl::errors::DecodeError>
            {
                #check_type
                #decode_arguments
                #decode_properties
                let #children = #node.children.as_ref()
                    .map(|lst| &lst[..]).unwrap_or(&[]);
                #decode_children
                #assign_extra
                Ok(#struct_expression)
            }
        }
    })
}

fn decode_scalar(val: &syn::Ident, ctx: &syn::Ident) -> TokenStream {
    quote!(::kfl::traits::DecodeScalar::decode(#val, #ctx))
}

fn check_type(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let name = crate::to_kebab_case(&s.object.ident.unraw());
    Ok(quote! {
        if let Some(type_name) = &#node.type_name {
            return Err(::kfl::errors::DecodeError::unexpected(
                       #ctx.span(&#node), "type name",
                       "no type name expected for this node"));
        }
        if #node.node_name.as_ref() != #name {
            return Err(::kfl::errors::DecodeError::unexpected(#ctx.span(&#node),
                       "node", format!("unexpected node `{}`",
                       #node.node_name.as_ref())));
        }
    })
}

// fn decode_specials(s: &Common, node: &syn::Ident)
//     -> syn::Result<TokenStream>
// {
//     let ctx = s.ctx;
//     let type_names = s.object.type_names.iter().flat_map(|type_name| {
//         let field = &type_name.field.tmp_name;
//         quote! {
//             let #field = if let Some(tn) = #node.type_name.as_ref() {
//                 tn.as_str()
//                     .parse()
//                     .map_err(|e|
//                         ::kfl::errors::DecodeError::conversion(tn, e))?
//             } else {
//                 return Err(::kfl::errors::DecodeError::missing(
//                            #ctx.span(&#node), "type name required"));
//             };
//         }
//     });
//     Ok(quote!())
// }

pub(crate) fn decode_arguments(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let mut decoder = Vec::new();
    let iter_args = syn::Ident::new("iter_args", Span::mixed_site());
    decoder.push(quote! {
        let mut #iter_args = #node.arguments.iter();
    });
    for argument in &s.object.arguments {
        let field = &argument.field.tmp_name;
        let val = syn::Ident::new("val", Span::mixed_site());
        let decode_scalar = decode_scalar(&val, ctx);
        match &argument.default {
            None => {
                let error = if argument.field.is_indexed() {
                    "additional argument is required".into()
                } else {
                    format!("additional argument `{}` is required", field.unraw())
                };
                decoder.push(quote! {
                    let #val =
                        #iter_args.next().ok_or_else(|| {
                            ::kfl::errors::DecodeError::missing(
                                #ctx.span(&#node), #error)
                        })?;
                    let #field = #decode_scalar?;
                });
            }
            Some(default_value) => {
                let default = if let Some(expr) = default_value {
                    quote!(#expr)
                } else {
                    quote!(::std::default::Default::default())
                };
                decoder.push(quote! {
                    let #field = #iter_args.next().map(|#val| #decode_scalar)
                            .transpose()?.unwrap_or_else(|| #default);
                });
            }
        }
    }
    if let Some(var_args) = &s.object.var_args {
        let field = &var_args.field.tmp_name;
        let val = syn::Ident::new("val", Span::mixed_site());
        let decode_scalar = decode_scalar(&val, ctx);
        decoder.push(quote! {
            let #field = #iter_args.map(|#val| #decode_scalar)
                                 .collect::<Result<_, _>>()?;
        });
    } else {
        decoder.push(quote! {
            if let Some(val) = #iter_args.next() {
                return Err(::kfl::errors::DecodeError::unexpected(
                        #ctx.span(&val.literal), "argument",
                        "unexpected argument"));
            }
        });
    }
    Ok(quote!(#(#decoder)*))
}

pub(crate) fn decode_properties(s: &Common, node: &syn::Ident)
    -> syn::Result<TokenStream>
{
    let mut declare_empty = Vec::new();
    let mut match_branches = Vec::new();
    let mut postprocess = Vec::new();

    let ctx = s.ctx;
    let val = syn::Ident::new("val", Span::mixed_site());
    let name = syn::Ident::new("name", Span::mixed_site());
    let name_str = syn::Ident::new("name_str", Span::mixed_site());

    for property in &s.object.properties {
        let field = &property.field.tmp_name;
        let prop_name = &property.name;
        let seen_name = format_ident!("seen_{}", field, span = Span::mixed_site());
        if false /* TODO property.flatten */ {
            declare_empty.push(quote! {
                let mut #field = ::std::default::Default::default();
            });
            match_branches.push(quote! {
                _ if ::kfl::traits::DecodePartial::
                    insert_property(&mut #field, #name, #val, #ctx)?
                => {}
            });
        } else {
            let decode_scalar = decode_scalar(&val, ctx);
            declare_empty.push(quote! {
                let mut #field = None;
                let mut #seen_name = false;
            });
            match_branches.push(quote! {
                #prop_name => {
                    #field = Some(#decode_scalar?);
                }
            });
            let req_msg = format!("property `{}` is required", prop_name);
            if let Some(value) = &property.default {
                let default = if let Some(expr) = value {
                    quote!(#expr)
                } else {
                    quote!(::std::default::Default::default())
                };
                postprocess.push(quote! {
                    let #field = #field.unwrap_or_else(|| #default);
                });
            } else {
                postprocess.push(quote! {
                    let #field = #field.ok_or_else(|| {
                        ::kfl::errors::DecodeError::missing(
                            #ctx.span(&#node), #req_msg)
                    })?;
                });
            }
        }
    }
    if let Some(var_props) = &s.object.var_props {
        let field = &var_props.field.tmp_name;
        let decode_scalar = decode_scalar(&val, ctx);
        declare_empty.push(quote! {
            let mut #field = Vec::new();
        });
        match_branches.push(quote! {
            #name_str => {
                let converted_name = #name_str.parse()
                    .map_err(|e| {
                        ::kfl::errors::DecodeError::conversion(#ctx.span(&#name), e)
                    })?;
                #field.push((
                    converted_name,
                    #decode_scalar?,
                ));
            }
        });
        postprocess.push(quote! {
            let #field = #field.into_iter().collect();
        });
    } else {
        match_branches.push(quote! {
            #name_str => {
                return Err(::kfl::errors::DecodeError::unexpected(
                    #ctx.span(&#name), "property",
                    format!("unexpected property `{}`",
                            #name_str.escape_default())));
            }
        });
    };
    Ok(quote! {
        #(#declare_empty)*
        for (#name, #val) in #node.properties.iter() {
            match #name.as_ref() {
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
//     };

//     let node = syn::Ident::new("node", Span::mixed_site());
//     let children = syn::Ident::new("children", Span::mixed_site());
//     let decode_arguments = decode_arguments(&common, &node)?;
//     let decode_properties = decode_properties(&common, &node)?;
//     let decode_children = decode_children(&common, &children,
//                                           Some(quote!(#ctx.span(&#node))))?;
//     Ok(quote! {
//         let mut #func = |#node: &::kfl::ast::Node,
//                          #ctx: &mut ::kfl::context::Context|
//         {
//             #decode_arguments
//             #decode_properties
//             let #children = #node.children.as_ref()
//                 .map(|lst| &lst[..]).unwrap_or(&[]);
//             #decode_children

//             Ok(#name)
//         };
//     })
// }

fn has_only_children(s: &Struct) -> bool {
    !s.has_arguments && !s.has_properties
    // && s.children.iter().all(|child| child.default.is_some())
}

fn decode_partial(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let mut branches = vec![quote! {
        if false {
            Ok(false)
        }
    }];
    for child_def in &s.object.children {
        let field = &child_def.field.from_self();
        let ty = &child_def.field.ty;
        branches.push(quote! {
            else if let Ok(true) = <#ty as ::kfl::traits::DecodePartial>
                ::decode_partial(&mut #field, #node, #ctx)
            {
                Ok(true)
            }
        });
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
//     for property in &s.object.properties {
//         let dest = &property.field.from_self();
//         let prop_name = &property.name;
//         if false /* TODO property.flatten */ {
//             match_branches.push(quote! {
//                 _ if ::kfl::traits::DecodePartial
//                     ::insert_property(&mut #dest, #name, #value, #ctx)?
//                 => Ok(true),
//             });
//         } else {
//             let decode_scalar = decode_scalar(&value, ctx, &property.decode);
//             match_branches.push(quote! {
//                 #prop_name => {
//                     #dest = Some(#decode_scalar?);
//                     Ok(true)
//                 }
//             });
//         }
//     }
//     Ok(quote! {
//         match &*#name {
//             #(#match_branches)*
//             _ => Ok(false),
//         }
//     })
// }

pub(crate) fn decode_children(s: &Common, children: &syn::Ident,
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
        let field = &child_def.field.tmp_name;
        let ty = &child_def.field.ty;
        match child_def.mode {
            ChildMode::Flatten => {
                declare_empty.push(quote! {
                    let mut #field = ::std::default::Default::default();
                });
                branches.push(quote! {
                    else if let Ok(true) = <#ty as ::kfl::traits::DecodePartial>
                        ::decode_partial(&mut #field, #child, #ctx) {
                        None
                    }
                });
            }
            ChildMode::Multi => {
                declare_empty.push(quote! {
                    let mut #field = Vec::new();
                });
                let ctx = &s.ctx;
                branches.push(quote! {
                    else if let Ok(true) = <Vec<<#ty as IntoIterator>::Item> as ::kfl::traits::DecodePartial>
                        ::decode_partial(&mut #field, #child, #ctx)
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
                        let #field = if #field.is_empty() {
                            #default
                        } else {
                            #field.into_iter().collect()
                        };
                    });
                } else {
                    postprocess.push(quote! {
                        let #field = #field.into_iter().collect();
                    });
                }
            }
            ChildMode::Normal => {
                declare_empty.push(quote! {
                    let mut #field = None;
                });
                branches.push(quote! {
                    else if let Ok(true) = <Option<#ty> as ::kfl::traits::DecodePartial>
                        ::decode_partial(&mut #field, #child, #ctx)
                    {
                        None
                    }
                });
                let req_msg = format!(
                    "child node for struct field `{}` is required",
                    &field.unraw().to_string());
                if let Some(default_value) = &child_def.default {
                    let default = if let Some(expr) = default_value {
                        quote!(#expr)
                    } else {
                        quote!(::std::default::Default::default())
                    };
                    postprocess.push(quote! {
                        let #field = #field.unwrap_or_else(|| #default);
                    });
                } else {
                    if let Some(span) = &err_span {
                        postprocess.push(quote! {
                            let #field = #field.ok_or_else(|| {
                                ::kfl::errors::DecodeError::Missing {
                                    span: #span.clone(),
                                    message: #req_msg.into(),
                                }
                            })?;
                        });
                    } else {
                        postprocess.push(quote! {
                            let #field = #field.ok_or_else(|| {
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
                #ctx.span(&#child), "node",
                format!("unexpected node `{}`",
                        #child.node_name.as_ref())));
            None
        }
    });
    Ok(quote! {
        #(#declare_empty)*
        #children.iter().flat_map(|#child| {
            #(#branches)*
        }).collect::<Result<(), ::kfl::errors::DecodeError>>()?;
        #(#postprocess)*
    })
}

pub(crate) fn assign_extra(s: &Common) -> syn::Result<TokenStream> {
    let items = s.object.extra_fields.iter().map(|field| {
        match field.kind {
            ExtraKind::Auto => {
                let name = &field.field.tmp_name;
                quote!(let #name = ::std::default::Default::default();)
            }
        }
    });
    Ok(quote!(#(#items)*))
}

pub fn emit_encode_struct(s: &Struct, partial: bool)
    -> syn::Result<TokenStream>
{
    let s_name = &s.ident;
    let node = syn::Ident::new("node", Span::mixed_site());
    let ctx = syn::Ident::new("ctx", Span::mixed_site());

    // TODO(rnarkk) merge
    let (_, type_gen, _) = s.generics.split_for_impl();
    let common_generics = s.generics.clone();
    let (impl_gen, _, bounds) = common_generics.split_for_impl();

    let common = Common {
        object: s,
        ctx: &ctx,
    };

    let declare_node = declare_node(&node, &s_name);
    // let encode_specials = encode_specials(&common, &node)?;
    let encode_arguments = encode_arguments(&common, &node, false)?;
    let encode_properties = encode_properties(&common, &node, false)?;
    let encode_children_normal = encode_children(
        &common, &node, Some(quote!(#ctx.span(&#node))))?;
    // let assign_extra = assign_extra(&common)?;

    let mut extra_traits = Vec::new();
    if partial {
        if has_only_children(&s) {
            let node = syn::Ident::new("node", Span::mixed_site());
            // let name = syn::Ident::new("name", Span::mixed_site());
            // let scalar = syn::Ident::new("scalar", Span::mixed_site());
            let encode_partial = encode_partial(&common, &node)?;
            // let insert_property = insert_property(&common, &name, &scalar)?;
            extra_traits.push(quote! {
                impl #impl_gen ::kfl::traits::EncodePartial
                    for #s_name #type_gen
                    #bounds
                {
                    fn encode_partial(
                        &self,
                        #node: &mut ::kfl::ast::Node,
                        #ctx: &mut ::kfl::context::Context)
                        -> Result<(), ::kfl::errors::EncodeError>
                    {
                        #encode_partial
                    }
                    // fn insert_property(&mut self,
                    //     #name: &Box<str>,
                    //     #scalar: &::kfl::ast::Scalar,
                    //     #ctx: &mut ::kfl::context::Context)
                    //     -> Result<bool, ::kfl::errors::EncodeError>
                    // {
                    //     #insert_property
                    // }
                }
            });
        } else {
            return Err(syn::Error::new(s.ident.span(), "not partial compatible"));
        }
    }
    Ok(quote! {
        #(#extra_traits)*
        impl #impl_gen ::kfl::traits::Encode for #s_name #type_gen #bounds {
            fn encode(&self, #ctx: &mut ::kfl::context::Context)
                -> Result<::kfl::ast::Node, ::kfl::errors::EncodeError>
            {
                #declare_node
                #encode_arguments
                #encode_properties
                #encode_children_normal
                // #assign_extra
                Ok(#node)
            }
        }
    })
}

fn declare_node(node: &syn::Ident, name: &syn::Ident) -> TokenStream {
    let name = crate::to_kebab_case(name);
    quote! { let mut #node = ::kfl::ast::Node::new(#name); }
}

pub(crate) fn encode_arguments(s: &Common, node: &syn::Ident, variant: bool)
    -> syn::Result<TokenStream>
{
    let ctx = s.ctx;
    let mut encoder = Vec::new();
    let scalar = syn::Ident::new("scalar", Span::mixed_site());
    // encoder.push(quote! {
    //     let mut #iter_args = #node.arguments.iter();
    // });
    for argument in &s.object.arguments {
        let field = if variant {
            let name = &argument.field.tmp_name;
            quote!(#name)
        } else {
            let name = argument.field.from_self();
            quote!(&#name)
        };
        let ty = &argument.field.ty;
        let encode_scalar = quote!(::kfl::traits::EncodeScalar::encode(
                                   #field, #ctx));
        match &argument.default {
            None => {
                // let error = if argument.field.is_indexed() {
                //     "additional argument is required".into()
                // } else {
                //     format!("additional argument `{}` is required", field.unraw())
                // };
                encoder.push(quote! {
                    #node.arguments.push(#encode_scalar?);
                    // let #val =
                    //     #iter_args.next().ok_or_else(|| {
                    //         ::kfl::errors::EncodeError::missing(
                    //             #ctx.span(&#node), #error)
                    //     })?;
                    // let #field = #encode_scalar?;
                });
            }
            Some(default_value) => {
                let default = if let Some(expr) = default_value {
                    quote!(#expr)
                } else {
                    quote!(::std::default::Default::default())
                };
                encoder.push(quote! {
                    let default: #ty = #default;
                    if &default != #field {
                        let #scalar = #encode_scalar?;
                        #node.arguments.push(#scalar);
                    }
                    // let #field = #iter_args.next().map(|#val| {
                    //     #encode_scalar
                    // }).transpose()?.unwrap_or_else(|| {
                    //     #default
                    // });
                });
            }
        }
    }
    if let Some(var_args) = &s.object.var_args {
        let field = &var_args.field.from_self();
        let scalar = syn::Ident::new("scalar", Span::mixed_site());
        let encode_scalar = quote!(::kfl::traits::EncodeScalar::encode(
                                   #scalar, #ctx));
        encoder.push(quote! {
            let args = #field.iter().map(|#scalar| #encode_scalar)
                    .collect::<Result<Vec<_>, _>>()?;
            #node.arguments.extend(args);
        });
    } else {
        // encoder.push(quote! {
        //     if let Some(scalar) = #field.into_iter().next() {
        //         return Err(::kfl::errors::EncodeError::unexpected(
        //                 #ctx.span(&scalar.literal), "argument",
        //                 "unexpected argument"));
        //     }
        // });
    }
    Ok(quote!(#(#encoder)*))
}

// TODO(rnarkk) named and unnamed
pub(crate) fn encode_properties(s: &Common, node: &syn::Ident, variant: bool)
    -> syn::Result<TokenStream>
{
    // let mut preprocess = Vec::new();
    let mut branches = Vec::new();

    let ctx = s.ctx;
    let scalar = syn::Ident::new("scalar", Span::mixed_site());
    // let props = syn::Ident::new("props", Span::mixed_site());
    // let declare_empty = quote!(let mut #props = Vec::new(););

    for property in &s.object.properties {
        let field = if variant {
            let name = &property.field.tmp_name;
            quote!(#name)
        } else {
            let name = property.field.from_self();
            quote!(&#name)
        };
        let name = &property.name;
        let ty = &property.field.ty;
        // let seen_name = format_ident!("seen_{}", field, span = Span::mixed_site());
        if false /* TODO property.flatten */ {
            // declare_empty.push(quote! {
            //     let mut #field = ::std::default::Default::default();
            // });
            // match_branches.push(quote! {
            //     _ if ::kfl::traits::EncodePartial::
            //         insert_property(&mut #field, #name, #val, #ctx)?
            //     => {}
            // });
        } else {
            let encode_scalar = quote!(::kfl::traits::EncodeScalar::encode(
                                       #field, #ctx));
            // let req_msg = format!("property `{}` is required", prop_name);
            if let Some(value) = &property.default {
                let default = if let Some(expr) = value {
                    quote!(#expr)
                } else {
                    quote!(::std::default::Default::default())
                };
                branches.push(quote! {
                    let default: #ty = #default;
                    if &default != #field {
                        let #scalar = #encode_scalar?;
                        #node.properties.insert(#name.to_string().into_boxed_str(), #scalar);
                    }
                });
            } else {
                // postprocess.push(quote! {
                //     let #field = #field.ok_or_else(|| {
                //         ::kfl::errors::EncodeError::missing(
                //             #ctx.span(&#node), #req_msg)
                //     })?;
                // });
                
                branches.push(quote! {
                    let #scalar = #encode_scalar?;
                    // let mut #seen_name = false;
                    #node.properties.insert(#name.to_string().into_boxed_str(), #scalar);
                });
            }
        }
    }
    if let Some(var_props) = &s.object.var_props {
        let field = &var_props.field.from_self();
        let scalar = syn::Ident::new("scalar", Span::mixed_site());
        let encode_scalar = quote!(::kfl::traits::EncodeScalar::encode(
                                   #scalar, #ctx));
        // declare_full.push(quote! {
        //     let mut #field = Vec::new();
        // });
        branches.push(quote! {
            // #name_str => {
            //     let converted_name = #name_str.parse()
            //         .map_err(|e| {
            //             ::kfl::errors::EncodeError::conversion(
            //                 #ctx.span(&#name), e)
            //         })?;
            //     #field.push((
            //         converted_name,
            //         #encode_scalar?,
            //     ));
            // }
            for (name, #scalar) in #field.iter() {
                #node.properties.insert(name.clone().into_boxed_str(),
                                        #encode_scalar?);
            }
        });
        // postprocess.push(quote! {
        //     let #field = #field.into_iter().collect();
        // });
    } else {
    //     match_branches.push(quote! {
    //         #name_str => {
    //             return Err(::kfl::errors::EncodeError::unexpected(
    //                 #ctx.span(&#name), "property",
    //                 format!("unexpected property `{}`",
    //                         #name_str.escape_default())));
    //         }
    //     });
    }
    Ok(quote! {
        // #(#preprocess)*
        #(#branches)*
        // #(#postprocess)*
    })
}

fn encode_partial(s: &Common, node: &syn::Ident) -> syn::Result<TokenStream> {
    let ctx = s.ctx;
    let mut branches = Vec::new();
    for child_def in &s.object.children {
        let field = &child_def.field.from_self();
        let ty = &child_def.field.ty;
        branches.push(quote! {
            let _ = <#ty as ::kfl::traits::EncodePartial>
                ::encode_partial(&#field, #node, #ctx)?;
        });
    }
    branches.push(quote!(Ok(())));
    Ok(quote!(#(#branches)*))
}

pub(crate) fn encode_children(s: &Common, node: &syn::Ident, _err_span: Option<TokenStream>)
    -> syn::Result<TokenStream>
{
    if s.object.children.is_empty() {
        return Ok(quote!());
    }
    // let mut declare_empty = Vec::new();
    let mut encodes = Vec::new();
    // let mut postprocess = Vec::new();

    let ctx = s.ctx;
    let children = syn::Ident::new("children", Span::mixed_site());
    let child = syn::Ident::new("child", Span::mixed_site());
    for child_def in &s.object.children {
        let field = &child_def.field.from_self();
        let ty = &child_def.field.ty;
        match child_def.mode {
            ChildMode::Flatten => {
                // declare_empty.push(quote! {
                //     let mut #field = ::std::default::Default::default();
                // });
                encodes.push(quote! {
                    let _ = <#ty as ::kfl::traits::EncodePartial>
                        ::encode_partial(&#field, &mut #node, #ctx);
                });
            }
            ChildMode::Multi => {
                // declare_empty.push(quote! {
                //     let mut #field = Vec::new();
                // });
                let ctx = &s.ctx;
                if let Some(default_value) = &child_def.default {
                    let default = if let Some(expr) = default_value {
                        quote!(#expr)
                    } else {
                        quote!(::std::default::Default::default())
                    };
                    encodes.push(quote! {
                        let default: #ty = #default;
                        if default != #field {
                            for #child in #field.iter() {
                                let #child = <<#ty as IntoIterator>::Item as ::kfl::traits::Encode>
                                ::encode(&#child, #ctx)?;
                                #children.push(#child);
                            }
                        }
                        // else if let Ok(true) = <Vec<<#ty as IntoIterator>::Item> as ::kfl::traits::EncodePartial>
                        //     ::encode_partial(&#field, &mut #output, #ctx)
                        // {
                        //     None
                        // }
                    });
                    // postprocess.push(quote! {
                    //     let #field = if #field.is_empty() {
                    //         #default
                    //     } else {
                    //         #field.into_iter().collect()
                    //     };
                    // });
                } else {
                    // postprocess.push(quote! {
                    //     let #field = #field.into_iter().collect();
                    // });
                    encodes.push(quote! {
                        for #child in #field.iter() {
                            let #child = <<#ty as IntoIterator>::Item as ::kfl::traits::Encode>
                            ::encode(&#child, #ctx)?;
                            #children.push(#child);
                        }
                    });
                }
            }
            ChildMode::Normal => {
                // declare_empty.push(quote! {
                //     let mut #field = None;
                // });
                // let req_msg = format!(
                //     "child node for struct field `{}` is required",
                //     &field.to_string());
                if let Some(default_value) = &child_def.default {
                    let default = if let Some(expr) = default_value {
                        quote!(#expr)
                    } else {
                        quote!(::std::default::Default::default())
                    };
                    encodes.push(quote! {
                        let default: #ty = #default;
                        if default != #field {
                            let #child = <#ty as ::kfl::traits::Encode>
                            ::encode(&#field, #ctx)?;
                            #children.push(#child);
                        }
                    });
                    // postprocess.push(quote! {
                    //     let #field = #field.unwrap_or_else(|| #default);
                    // });
                } else {
                    // if let Some(span) = &err_span {
                    //     postprocess.push(quote! {
                    //         let #field = #field.ok_or_else(|| {
                    //             ::kfl::errors::EncodeError::Missing {
                    //                 span: #span.clone(),
                    //                 message: #req_msg.into(),
                    //             }
                    //         })?;
                    //     });
                    // } else {
                    //     postprocess.push(quote! {
                    //         let #field = #field.ok_or_else(|| {
                    //             ::kfl::errors::EncodeError::MissingNode {
                    //                 message: #req_msg.into(),
                    //             }
                    //         })?;
                    //     });
                    // }
                    encodes.push(quote! {
                        let #child = <#ty as ::kfl::traits::Encode>
                        ::encode(&#field, #ctx)?;
                        #children.push(#child);
                    });
                }
            }
        }
    }
    // TODO(rnarkk) return Err?
    // encodes.push(quote! {
    //     else {
    //         #ctx.emit_error(::kfl::errors::EncodeError::unexpected(
    //             #ctx.span(&#child), "node",
    //             format!("unexpected node `{}`",
    //                     #child.node_name.as_ref())));
    //         None
    //     }
    // });
    Ok(quote! {
        // #(#declare_empty)*
        let mut #children = Vec::new();
        #(#encodes)*
        if !#children.is_empty() {
            #node.children = Some(#children);
        }
        // #children.iter().flat_map(|#child| {
        //     #(#encodes)*
        // }).collect::<Result<(), ::kfl::errors::EncodeError>>()?;
        // #(#postprocess)*
    })
}
