use std::{fs::File, io::BufReader};

use anyhow::Result;
use data_definition::DataDefinition;
use id3::create_tree;

mod data_definition;
mod decision_tree;
mod id3;

fn main() -> Result<()> {
    let f = File::open("model-xstigl00.txt")?;

    let data = DataDefinition::parse(BufReader::new(f))?;
    let tree = create_tree(&data);
    println!("digraph {{");
    println!("{tree:+}");
    println!("}}");

    Ok(())
}
