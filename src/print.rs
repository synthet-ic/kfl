//! Display implementation for ast

use core::fmt::Display;

use crate::ast::{Node, Scalar, Literal, Integer, Decimal};

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO(rnarkk)
        // if let Some(typ) = &self.type_name {
        //     write!(f, "({})", &typ)?;
        // }
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
            Literal::Int(value) => write!(f, "{}", value),
            Literal::Decimal(value) => write!(f, "{}", value),  // TODO
            Literal::String(value) => write!(f, "\"{}\"", value)
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
        // match self.0 {
        //     Radix::Bin => write!(f, "{:b}", self.1.to_string().as_str()),
        //     Radix::Oct => write!(f, "{:o}", self.1.to_string().as_ref()),
        //     Radix::Dec => write!(f, "{}", self.1),
        //     Radix::Hex => write!(f, "{:x}", self.1.to_string().as_ref()),
        // }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
