use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use anyhow::{anyhow, bail, Error, Result};

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
            match line.trim() {
                "" => {}
                "attributes {" => {
                    data.parse_with(Self::parse_attribute, &mut lines)?
                }
                "classes {" => {
                    data.parse_with(Self::parse_class, &mut lines)?
                }
                "objects {" => {
                    data.parse_with(Self::parse_object, &mut lines)?
                }
                _ => bail!("Unexpected line in input: {line}"),
            }
        }
        todo!()
    }

    fn parse_with<F, I>(&mut self, f: F, lines: &mut I) -> Result<()>
    where
        I: Iterator<Item = Result<String>>,
        F: Fn(&mut Self, &str) -> Result<()>,
    {
        while let Some(line) = lines.next() {
            let line = line?;
            if line == "}" {
                break;
            }
            f(self, &line)?;
        }
        Ok(())
    }

    fn parse_attribute(&mut self, line: &str) -> Result<()> {
        let mut spl = line.split_ascii_whitespace();
        let name = spl.next().ok_or(anyhow!("missing attribute name"))?;
        if spl.next().ok_or(anyhow!("missing :"))? != ":" {
            bail!("Expected :");
        }
        
        let mut values = HashSet::new();
        values.extend(spl.map(str::to_string));

        self.attributes.push(Attribute {
            name: name.to_owned(),
            values,
        });

        Ok(())
    }

    fn parse_class(&mut self, line: &str) -> Result<()> {
        todo!()
    }

    fn parse_object(&mut self, line: &str) -> Result<()> {
        todo!()
    }
}
