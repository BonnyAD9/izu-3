use std::collections::HashMap;

use crate::data_definition::Object;

pub struct DecNode<'a> {
    id: i32,
    attribute: &'a str,
    info_gains: HashMap<&'a str, f64>,
    children: Vec<DecNodeChild<'a>>,
}

pub struct DecNodeChild<'a> {
    attr_class: &'a str,
    objects: Vec<&'a Object>,
    child: Option<DecNode<'a>>,
}
