use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use anyhow::{bail, Error, Result};

#[derive(Default)]
pub struct DataDefinition {
    attributes: Vec<Attribute>,
    classes: HashSet<String>,
    objects: Vec<Object>,
}

pub struct Attribute {
    name: String,
    values: HashSet<String>,
}

pub struct Object {
    id: i32,
    class: String,
    attributes: HashMap<String, String>,
}

impl DataDefinition {
    pub fn parse<R>(mut input: R) -> Result<DataDefinition>
    where
        R: BufRead,
    {
        let mut lines = input.lines().map(|a| a.map_err(Error::new));
        let mut data = DataDefinition::default();
        while let Some(line) = lines.next() {
            let line = line?;
            match line.as_str() {
                "" => {}
                "attributes {" => data.parse_attributes(&mut lines)?,
                "classes {" => data.parse_classes(&mut lines)?,
                "objects {" => data.parse_objects(&mut lines)?,
                _ => bail!("Unexpected line in input: {line}"),
            }
        }
        todo!()
    }

    fn parse_attributes<I>(&mut self, lines: &mut I) -> Result<()>
    where
        I: Iterator<Item = Result<String>>,
    {
        todo!()
    }

    fn parse_classes<I>(&mut self, lines: &mut I) -> Result<()>
    where
        I: Iterator<Item = Result<String>>,
    {
        todo!()
    }

    fn parse_objects<I>(&mut self, lines: &mut I) -> Result<()>
    where
        I: Iterator<Item = Result<String>>,
    {
        todo!()
    }
}
