//! Display implementation for ast

use core::fmt::{self, Display};

use crate::ast::{Node, Scalar, Integer, Decimal};

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(typ) = &self.type_name {
            write!(f, "({})", &typ)?;
        }
        write!(f, "{}", &self.literal)
    }
}

// TODO(rnarkk) Remove
impl Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
        // match self.0 {
        //     2 => write!(f, "{:b}", self.1.to_string().as_str()),
        //     8 => write!(f, "{:o}", self.1.to_string().as_ref()),
        //     10 => write!(f, "{}", self.1),
        //     16 => write!(f, "{:x}", self.1.to_string().as_ref()),
        // }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
