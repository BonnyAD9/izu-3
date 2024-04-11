use std::{
    env,
    fs::File,
    io::{self, BufReader, Write},
};

use anyhow::Result;
use args::Args;
use data_definition::DataDefinition;
use id3::create_tree;

mod args;
mod data_definition;
mod decision_tree;
mod id3;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let args = Args::parse(args.iter().map(|a| a.as_str()).skip(1))?;

    let data = if let Some(i) = args.input {
        DataDefinition::parse(BufReader::new(File::open(i)?))?
    } else {
        DataDefinition::parse(BufReader::new(io::stdin()))?
    };

    let tree = create_tree(&data);

    if let Some(o) = args.md_output {
        let mut f = File::create(o)?;
        write!(f, "{tree}")?;
    } else {
        println!("{tree}");
    }

    if let Some(o) = args.dot_output {
        let mut f = File::create(o)?;
        writeln!(f, "digraph{{\n{tree:+}\n}}")?;
    } else {
        println!("digraph{{\n{tree:+}\n}}");
    }

    Ok(())
}
