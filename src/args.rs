use std::{path::PathBuf, str::FromStr};

use anyhow::{bail, Result};

pub struct Args<'a> {
    pub input: Option<&'a str>,
    pub md_output: Option<&'a str>,
    pub dot_output: Option<&'a str>,
}

trait ArgIterator<T> {
    fn get(&mut self) -> Result<T>;
}

impl<'a, I> ArgIterator<&'a str> for I
where
    I: Iterator<Item = &'a str>,
{
    fn get(&mut self) -> Result<&'a str> {
        let Some(i) = self.next() else {
            bail!("Expected another argument");
        };
        Ok(i)
    }
}

impl<'a> Args<'a> {
    pub fn parse<I>(mut args: I) -> Result<Args<'a>>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut res = Args {
            input: None,
            md_output: None,
            dot_output: None,
        };

        while let Some(arg) = args.next() {
            match arg {
                "-m" | "--md" | "--markdown" => {
                    res.md_output = Some(args.get()?)
                }
                "-d" | "--dot" => res.dot_output = Some(args.get()?),
                "-i" | "--input" => res.input = Some(args.get()?),
                a => res.input = Some(a),
            }
        }

        if res.md_output.is_none() && res.dot_output.is_none() {
            bail!("Invalid outputs, only one may be stdout");
        }

        Ok(res)
    }
}
