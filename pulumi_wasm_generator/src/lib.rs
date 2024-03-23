mod schema;
mod model;
mod output;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

pub fn generate_files(path: &Path, result_path: &Path) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package: schema::Package = serde_json::from_reader(reader)?;

    println!("{:#?}", package);

    Ok(())
}