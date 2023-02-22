use std::mem;

use proc_macro2::{TokenStream, Span};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned
};

use crate::kw;

pub enum Definition {
    UnitStruct(Struct),
    TupleStruct(Struct),
    NewType(NewType),
    Struct(Struct),
    Enum(Enum),
}

pub enum VariantKind {
    Unit,
    Nested { ty: syn::Ident },
    Tuple(Struct),
    Named(Struct),
}

#[derive(Debug, Clone)]
pub enum FieldMode {
    Argument,
    Property { name: Option<String> },
    Arguments,
    Properties,
    Children,
    Child,
    Flatten,
}

#[derive(Debug)]
pub enum Attr {
    Skip,
    FieldMode(FieldMode),
    Unwrap(FieldAttrs),
    Default(Option<syn::Expr>),
}

#[derive(Debug, Clone)]
pub struct FieldAttrs {
    pub mode: Option<FieldMode>,
    pub unwrap: Option<Box<FieldAttrs>>,
    pub default: Option<Option<syn::Expr>>,
}

#[derive(Debug, Clone)]
pub struct VariantAttrs {
    pub skip: bool,
}

#[derive(Clone)]
pub enum AttrAccess {
    Indexed(usize),
    Named(syn::Ident),
}

#[derive(Clone)]
pub struct Field {
    pub span: Span,
    pub attr: AttrAccess,
    pub tmp_name: syn::Ident,
    pub ty: syn::Type
}

pub struct Arg {
    pub field: Field,
    pub default: Option<Option<syn::Expr>>,
}

pub struct VarArgs {
    pub field: Field,
}

pub struct Prop {
    pub field: Field,
    pub name: String,
    pub default: Option<Option<syn::Expr>>,
}

pub struct VarProps {
    pub field: Field,
}

pub enum ChildMode {
    Normal,
    Multi,
    Flatten,
}

pub struct Child {
    pub field: Field,
    pub mode: ChildMode,
    pub unwrap: Option<Box<FieldAttrs>>,
    pub default: Option<Option<syn::Expr>>,
}

pub enum ExtraKind {
    Auto,
}

pub struct ExtraField {
    pub field: Field,
    pub kind: ExtraKind,
}

#[derive(Clone)]
pub struct TraitProps {
    pub span_type: Option<syn::Type>,
}

pub struct Struct {
    pub ident: syn::Ident,
    pub trait_props: TraitProps,
    pub generics: syn::Generics,
    pub arguments: Vec<Arg>,
    pub var_args: Option<VarArgs>,
    pub properties: Vec<Prop>,
    pub var_props: Option<VarProps>,
    pub has_arguments: bool,
    pub has_properties: bool,
    pub children: Vec<Child>,
    pub extra_fields: Vec<ExtraField>,
}

pub struct StructBuilder {
    pub ident: syn::Ident,
    pub trait_props: TraitProps,
    pub generics: syn::Generics,
    pub arguments: Vec<Arg>,
    pub var_args: Option<VarArgs>,
    pub properties: Vec<Prop>,
    pub var_props: Option<VarProps>,
    pub children: Vec<Child>,
    pub extra_fields: Vec<ExtraField>,
}

pub struct NewType {
    pub ident: syn::Ident,
    pub trait_props: TraitProps,
    pub generics: syn::Generics,
}

pub struct Variant {
    pub ident: syn::Ident,
    pub name: String,
    pub kind: VariantKind,
}

pub struct Enum {
    pub ident: syn::Ident,
    pub trait_props: TraitProps,
    pub generics: syn::Generics,
    pub variants: Vec<Variant>,
}

impl TraitProps {
    fn pick_from(attrs: &mut Vec<(Attr, Span)>) -> TraitProps {
        let props = TraitProps {
            span_type: None,
        };
        for attr in mem::take(attrs) {
            attrs.push(attr)
        }
        props
    }
}

fn err_pair(s1: &Field, s2: &Field, t1: &str, t2: &str)
    -> syn::Error
{
    let mut err = syn::Error::new(s1.span, t1);
    err.combine(syn::Error::new(s2.span, t2));
    return err;
}

impl Variant {
    fn new(ident: syn::Ident, _attrs: VariantAttrs, kind: VariantKind)
        -> syn::Result<Self>
    {
        let name = heck::ToKebabCase::to_kebab_case(&ident.unraw().to_string()[..]);
        Ok(Variant {
            ident,
            name,
            kind,
        })
    }
}

impl Enum {
    fn new(ident: syn::Ident, attrs: Vec<syn::Attribute>,
           generics: syn::Generics,
           src_variants: impl Iterator<Item = syn::Variant>)
        -> syn::Result<Self>
    {
        let mut attrs = parse_attr_list(&attrs);
        let trait_props = TraitProps::pick_from(&mut attrs);
        if !attrs.is_empty() {
            for (_, span) in attrs {
                emit_error!(span, "unexpected container attribute");
            }
        }

        let mut variants = Vec::new();
        for var in src_variants {
            let mut attrs = VariantAttrs::new();
            attrs.update(parse_attr_list(&var.attrs));
            if attrs.skip {
                continue;
            }
            let kind = match var.fields {
                syn::Fields::Named(n) => {
                    Struct::new(var.ident.clone(),
                                trait_props.clone(),
                                generics.clone(),
                                n.named.into_iter())
                    .map(VariantKind::Named)?
                }
                syn::Fields::Unnamed(u) => {
                    let tup = Struct::new(
                        var.ident.clone(),
                        trait_props.clone(),
                        generics.clone(),
                        u.unnamed.into_iter(),
                    )?;
                    if tup.all_fields().len() == 1
                        && tup.extra_fields.len() == 1
                        && matches!(tup.extra_fields[0].kind, ExtraKind::Auto)
                    {
                        // Single tuple variant without any defition means
                        // the first field inside is meant to be full node
                        // parser.
                        VariantKind::Nested { ty: tup.ident.clone() }
                    } else {
                        VariantKind::Tuple(tup)
                    }
                }
                syn::Fields::Unit => {
                    VariantKind::Unit
                }
            };
            variants.push(Variant::new(var.ident, attrs, kind)?);
        }
        Ok(Enum {
            ident,
            trait_props,
            generics,
            variants,
        })
    }
}

impl StructBuilder {
    pub fn new(ident: syn::Ident,
               trait_props: TraitProps,
               generics: syn::Generics)
        -> Self
    {
        StructBuilder {
            ident,
            trait_props,
            generics,
            arguments: Vec::new(),
            var_args: None::<VarArgs>,
            properties: Vec::new(),
            var_props: None::<VarProps>,
            children: Vec::new(),
            extra_fields: Vec::new(),
        }
    }
    pub fn build(self) -> Struct {
        Struct {
            ident: self.ident,
            trait_props: self.trait_props,
            generics: self.generics,
            has_arguments:
                !self.arguments.is_empty() || self.var_args.is_some(),
            has_properties:
                !self.properties.is_empty() || self.var_props.is_some(),
            arguments: self.arguments,
            var_args: self.var_args,
            properties: self.properties,
            var_props: self.var_props,
            children: self.children,
            extra_fields: self.extra_fields,
        }
    }
    pub fn add_field(&mut self, field: Field, attrs: &FieldAttrs)
        -> syn::Result<&mut Self>
    {
        match &attrs.mode {
            Some(FieldMode::Argument) => {
                if let Some(prev) = &self.var_args {
                    return Err(err_pair(&field, &prev.field,
                        "extra `argument` after capture all `arguments`",
                        "capture all `arguments` is defined here"));
                }
                self.arguments.push(Arg {
                    field,
                    default: attrs.default.clone(),
                });
            }
            Some(FieldMode::Arguments) => {
                if let Some(prev) = &self.var_args {
                    return Err(err_pair(&field, &prev.field,
                        "only single `arguments` allowed",
                        "previous `arguments` is defined here"));
                }
                self.var_args = Some(VarArgs {
                    field,
                });
            }
            Some(FieldMode::Property { name }) => {
                if let Some(prev) = &self.var_props {
                    return Err(err_pair(&field, &prev.field,
                        "extra `property` after capture all `properties`",
                        "capture all `properties` is defined here"));
                }
                let name = match (name, &field.attr) {
                    (Some(name), _) => name.clone(),
                    (None, AttrAccess::Named(name))
                    => heck::ToKebabCase::to_kebab_case(&name.unraw().to_string()[..]),
                    (None, AttrAccess::Indexed(_)) => {
                        return Err(syn::Error::new(field.span,
                            "property must be named, try \
                             `property(name=\"something\")"));
                    }
                };
                self.properties.push(Prop {
                    field,
                    name,
                    default: attrs.default.clone(),
                });
            }
            Some(FieldMode::Properties) => {
                if let Some(prev) = &self.var_props {
                    return Err(err_pair(&field, &prev.field,
                        "only single `properties` is allowed",
                        "previous `properties` is defined here"));
                }
                self.var_props = Some(VarProps {
                    field,
                });
            }
            Some(FieldMode::Child) => {
                self.children.push(Child {
                    field,
                    mode: ChildMode::Normal,
                    unwrap: attrs.unwrap.clone(),
                    default: attrs.default.clone(),
                });
            }
            Some(FieldMode::Children) => {
                self.children.push(Child {
                    field,
                    mode: ChildMode::Multi,
                    unwrap: attrs.unwrap.clone(),
                    default: attrs.default.clone(),
                });
            }
            Some(FieldMode::Flatten) => {
                self.children.push(Child {
                    field: field.clone(),
                    mode: ChildMode::Flatten,
                    unwrap: None,
                    default: None,
                });
            }
            None => {
                self.extra_fields.push(ExtraField {
                    field,
                    kind: ExtraKind::Auto,
                });
            }
        }
        return Ok(self);
    }
}

impl Struct {
    fn new(ident: syn::Ident, trait_props: TraitProps, generics: syn::Generics,
           fields: impl Iterator<Item = syn::Field>)
        -> syn::Result<Self>
    {
        let mut bld = StructBuilder::new(ident, trait_props, generics);
        for (idx, fld) in fields.enumerate() {
            let mut attrs = FieldAttrs::new();
            attrs.update(parse_attr_list(&fld.attrs));
            let field = Field::new(&fld, idx);
            bld.add_field(field, &attrs)?;
        }

        Ok(bld.build())
    }
    pub fn all_fields(&self) -> Vec<&Field> {
        let mut res = Vec::new();
        res.extend(self.arguments.iter().map(|a| &a.field));
        res.extend(self.var_args.iter().map(|a| &a.field));
        res.extend(self.properties.iter().map(|p| &p.field));
        res.extend(self.var_props.iter().map(|p| &p.field));
        res.extend(self.children.iter().map(|c| &c.field));
        res.extend(self.extra_fields.iter().map(|f| &f.field));
        return res;
    }
}

impl Parse for Definition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = input.call(syn::Attribute::parse_outer)?;
        let ahead = input.fork();
        let _vis: syn::Visibility = ahead.parse()?;

        let lookahead = ahead.lookahead1();
        if lookahead.peek(syn::Token![struct]) {
            let item: syn::ItemStruct = input.parse()?;
            attrs.extend(item.attrs);

            let mut attrs = parse_attr_list(&attrs);
            let trait_props = TraitProps::pick_from(&mut attrs);
            if !attrs.is_empty() {
                for (_, span) in attrs {
                    emit_error!(span, "unexpected container attribute");
                }
            }

            match item.fields {
                syn::Fields::Named(n) => {
                    Struct::new(item.ident, trait_props, item.generics,
                                n.named.into_iter())
                    .map(Definition::Struct)
                }
                syn::Fields::Unnamed(u) => {
                    let tup = Struct::new(
                        item.ident.clone(),
                        trait_props.clone(),
                        item.generics.clone(),
                        u.unnamed.into_iter(),
                    )?;
                    if tup.all_fields().len() == 1
                        && tup.extra_fields.len() == 1
                        && matches!(tup.extra_fields[0].kind, ExtraKind::Auto)
                    {
                        Ok(Definition::NewType(NewType {
                            ident: item.ident,
                            trait_props,
                            generics: item.generics,
                            // option: tup.extra_fields[0].option,
                        }))
                    } else {
                        Ok(Definition::TupleStruct(tup))
                    }
                }
                syn::Fields::Unit => {
                    Struct::new(item.ident, trait_props, item.generics,
                                Vec::new().into_iter())
                    .map(Definition::UnitStruct)
                }
            }
        } else if lookahead.peek(syn::Token![enum]) {
            let item: syn::ItemEnum = input.parse()?;
            attrs.extend(item.attrs);
            Enum::new(item.ident, attrs, item.generics,
                      item.variants.into_iter())
                .map(Definition::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

impl FieldAttrs {
    fn new() -> FieldAttrs {
        FieldAttrs {
            mode: None,
            unwrap: None,
            default: None,
        }
    }
    fn update(&mut self, attrs: impl IntoIterator<Item=(Attr, Span)>) {
        use Attr::*;

        for (attr, span) in attrs {
            match attr {
                FieldMode(mode) => {
                    if self.mode.is_some() {
                        emit_error!(span,
                            "only single attribute that defines mode of the \
                            field is allowed. Perhaps you mean `unwrap`?");
                    }
                    self.mode = Some(mode);
                }
                Unwrap(val) => {
                    if self.unwrap.is_some() {
                        emit_error!(span, "`unwrap` specified twice");
                    }
                    self.unwrap = Some(Box::new(val));
                }
                Default(value) => {
                    if self.default.is_some() {
                        emit_error!(span,
                            "only single default is allowed");
                    }
                    self.default = Some(value);
                }
                _ => emit_error!(span,
                    "this attribute is not supported on fields"),
            }
        }
    }
}

impl VariantAttrs {
    fn new() -> VariantAttrs {
        VariantAttrs {
            skip: false,
        }
    }
    fn update(&mut self, attrs: impl IntoIterator<Item=(Attr, Span)>) {
        use Attr::*;

        for (attr, span) in attrs {
            match attr {
                Skip => self.skip = true,
                _ => emit_error!(span, "not supported on enum variants"),
            }
        }
    }
}

fn parse_attr_list(attrs: &[syn::Attribute]) -> Vec<(Attr, Span)> {
    let mut all = Vec::new();
    for attr in attrs {
        if matches!(attr.style, syn::AttrStyle::Outer) &&
            attr.path.is_ident("kfl")

        {
            match attr.parse_args_with(parse_attrs) {
                Ok(attrs) => all.extend(attrs),
                Err(e) => emit_error!(e),
            }
        }
    }
    return all;
}

fn parse_attrs(input: ParseStream)
    -> syn::Result<impl IntoIterator<Item=(Attr, Span)>>
{
    Punctuated::<_, syn::Token![,]>::parse_terminated_with(
        input, Attr::parse)
}

impl Attr {
    fn parse(input: ParseStream) -> syn::Result<(Self, Span)> {
        Self::_parse(input).map(|a| (a, input.span()))
    }
    fn _parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::argument) {
            let _kw: kw::argument = input.parse()?;
            Ok(Attr::FieldMode(FieldMode::Argument))
        } else if lookahead.peek(kw::arguments) {
            let _kw: kw::arguments = input.parse()?;
            Ok(Attr::FieldMode(FieldMode::Arguments))
        } else if lookahead.peek(kw::property) {
            let _kw: kw::property = input.parse()?;
            let mut name = None;
            if !input.is_empty() && !input.lookahead1().peek(syn::Token![,]) {
                let parens;
                syn::parenthesized!(parens in input);
                let lookahead = parens.lookahead1();
                if lookahead.peek(kw::name) {
                    let _kw: kw::name = parens.parse()?;
                    let _eq: syn::Token![=] = parens.parse()?;
                    let name_lit: syn::LitStr = parens.parse()?;
                    name = Some(name_lit.value());
                } else {
                    return Err(lookahead.error())
                }
            }
            Ok(Attr::FieldMode(FieldMode::Property { name }))
        } else if lookahead.peek(kw::properties) {
            let _kw: kw::properties = input.parse()?;
            Ok(Attr::FieldMode(FieldMode::Properties))
        } else if lookahead.peek(kw::children) {
            let _kw: kw::children = input.parse()?;
            if !input.is_empty() && !input.lookahead1().peek(syn::Token![,]) {
                let parens;
                syn::parenthesized!(parens in input);
                let lookahead = parens.lookahead1();
                if lookahead.peek(kw::name) {
                    let _kw: kw::name = parens.parse()?;
                    let _eq: syn::Token![=] = parens.parse()?;
                } else {
                    return Err(lookahead.error())
                }
            }
            Ok(Attr::FieldMode(FieldMode::Children))
        } else if lookahead.peek(kw::child) {
            let _kw: kw::child = input.parse()?;
            Ok(Attr::FieldMode(FieldMode::Child))
        } else if lookahead.peek(kw::unwrap) {
            let _kw: kw::unwrap = input.parse()?;
            let parens;
            syn::parenthesized!(parens in input);
            let mut attrs = FieldAttrs::new();
            let chunk = parens.call(parse_attrs)?;
            attrs.update(chunk);
            Ok(Attr::Unwrap(attrs))
        } else if lookahead.peek(kw::skip) {
            let _kw: kw::skip = input.parse()?;
            Ok(Attr::Skip)
        } else if lookahead.peek(kw::flatten) {
            let _kw: kw::flatten = input.parse()?;
            Ok(Attr::FieldMode(FieldMode::Flatten))
        } else if lookahead.peek(kw::default) {
            let _kw: kw::default = input.parse()?;
            if !input.is_empty() && !input.lookahead1().peek(syn::Token![,]) {
                let _eq: syn::Token![=] = input.parse()?;
                let value: syn::Expr = input.parse()?;
                Ok(Attr::Default(Some(value)))
            } else {
                Ok(Attr::Default(None))
            }
        } else {
            Err(lookahead.error())
        }
    }
}

impl Field {
    // pub fn new_named(name: &syn::Ident, ty: &syn::Type) -> Field {
    //     Field {
    //         span: name.span(),
    //         attr: AttrAccess::Named(name.clone()),
    //         tmp_name: name.clone(),
    //         ty: ty.clone()
    //     }
    // }
    fn new(field: &syn::Field, idx: usize) -> Field {
        field.ident.as_ref()
            .map(|id| Field {
                span: field.span(),
                attr: AttrAccess::Named(id.clone()),
                tmp_name: id.clone(),
                ty: field.ty.clone()
            })
            .unwrap_or_else(|| Field {
                span: field.span(),
                attr: AttrAccess::Indexed(idx),
                tmp_name: syn::Ident::new(
                    &format!("field{}", idx),
                    Span::mixed_site(),
                ),
                ty: field.ty.clone()
            })
    }
    pub fn from_self(&self) -> TokenStream {
        match &self.attr {
            AttrAccess::Indexed(idx) => quote!(self.#idx),
            AttrAccess::Named(name) => quote!(self.#name),
        }
    }
    pub fn is_indexed(&self) -> bool {
        matches!(self.attr, AttrAccess::Indexed(_))
    }
    pub fn as_index(&self) -> Option<usize> {
        match &self.attr {
            AttrAccess::Indexed(idx) => Some(*idx),
            AttrAccess::Named(_) => None,
        }
    }
    pub fn as_assign_pair(&self) -> Option<TokenStream> {
        match &self.attr {
            AttrAccess::Indexed(_) => None,
            AttrAccess::Named(n) if n == &self.tmp_name => Some(quote!(#n)),
            AttrAccess::Named(n) => {
                let tmp_name = &self.tmp_name;
                Some(quote!(#n: #tmp_name))
            }
        }
    }
}
