use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use anyhow::{anyhow, bail, Error, Result};

#[derive(Default)]
pub struct DataDefinition {
    pub attributes: Vec<Attribute>,
    pub classes: HashSet<String>,
    pub objects: Vec<Object>,
}

pub struct Attribute {
    pub name: String,
    pub values: HashSet<String>,
}

pub struct Object {
    pub id: i32,
    pub class: String,
    pub attributes: HashMap<String, String>,
}

impl DataDefinition {
    pub fn parse<R>(input: R) -> Result<DataDefinition>
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

        Ok(data)
    }

    fn parse_with<F, I>(&mut self, f: F, lines: &mut I) -> Result<()>
    where
        I: Iterator<Item = Result<String>>,
        F: Fn(&mut Self, &str) -> Result<()>,
    {
        for line in lines {
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
        self.classes.insert(line.trim().to_string());
        Ok(())
    }

    fn parse_object(&mut self, line: &str) -> Result<()> {
        let mut spl = line.split_ascii_whitespace();
        let id = spl.next().ok_or(anyhow!("missing attribute name"))?;
        let class = spl.next().ok_or(anyhow!("missing attribute name"))?;
        let id: i32 = id.parse()?;

        let mut attributes = HashMap::new();

        attributes.extend(
            self.attributes
                .iter()
                .zip(spl)
                .map(|(k, v)| (k.name.to_owned(), v.to_owned())),
        );

        self.objects.push(Object {
            id,
            class: class.to_string(),
            attributes,
        });

        Ok(())
    }
}
