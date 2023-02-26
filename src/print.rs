//! Display implementation for ast

use core::fmt::{self, Display};

use crate::ast::{Node, Scalar};

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
