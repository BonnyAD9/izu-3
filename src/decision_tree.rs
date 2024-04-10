use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::data_definition::Object;

pub struct DecNode<'a> {
    pub id: i32,
    pub attribute: &'a str,
    pub info_gains: HashMap<&'a str, f64>,
    pub children: Vec<DecNodeChild<'a>>,
}

pub struct DecNodeChild<'a> {
    pub attr_class: &'a str,
    pub objects: Vec<&'a Object>,
    pub child: Option<DecNode<'a>>,
    pub target_id: i32,
}

impl<'a> Display for DecNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            write!(f, "TODO")?;
            return Ok(());
        }
        write!(f, "node{} [label=\"{}|{{", self.id, self.attribute)?;
        for (i, (n, v)) in self.info_gains.iter().enumerate() {
            if i != 0 {
                write!(f, "|")?;
            }
            write!(f, "{n}={v}")?;
        }
        write!(f, "}}\"]\n")?;

        for c in &self.children {
            write!(f, "node{} -> {c}", self.id)?;
        }
        Ok(())
    }
}

impl<'a> Display for DecNodeChild<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            return write!(f, "TODO");
        }

        write!(f, "node{} [label=\"{} {{", self.target_id, self.attr_class)?;
        for (i, o) in self.objects.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", o.id)?;
        }
        write!(f, "}}\"]\n")?;

        if let Some(node) = &self.child {
            write!(f, "{}", node)?;
        } else {
            write!(f, "node{} [label=\"", self.target_id)?;
            let classes: HashSet<_> =
                self.objects.iter().map(|o| o.class.as_str()).collect();
            for (i, c) in classes.iter().enumerate() {
                if i != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{c}")?;
            }
            write!(f, "\"]")?;
        }

        Ok(())
    }
}
