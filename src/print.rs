//! Display implementation for ast

use std::fmt::Display;

use crate::ast::{Node, Scalar, Literal};

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(typ) = &self.type_name {
            write!(f, "({})", &typ)?;
        }
        write!(f, "{}", &self.node_name)?;
        for scalar in self.arguments.iter() {
            write!(f, " {}", &scalar)?;
        }
        for (name, scalar) in self.properties.iter() {
            write!(f, " {}={}", name, &scalar)?;
        }
        if let Some(children) = &self.children {
            write!(f, " {{")?;
            for child in children.iter() {
                write!(f, "\n  {}", child)?;
            }
            write!(f, "\n}}")
        } else {
            write!(f, "")
        }
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(typ) = &self.type_name {
            write!(f, "({})", &typ)?;
        }
        write!(f, "{}", &self.literal)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Null => write!(f, "null"),
            Literal::Bool(value) => write!(f, "{}", value),
            Literal::Int(value) => write!(f, "{}", 1),  // TODO
            Literal::Decimal(value) => write!(f, "{}", 1),  // TODO
            Literal::String(value) => write!(f, "\"{}\"", value)
        }
    }
}
