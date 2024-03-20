use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Type {
    r#type: Option<String>,
    #[serde(rename = "$ref")]
    r#ref: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Property {
    #[serde(flatten)]
    r#type: Type,
}

#[derive(Deserialize, Debug)]
struct ObjectType {
    description: Option<String>,
    r#type: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Resource {
    #[serde(flatten)]
    object_type: ObjectType,
    #[serde(default)]
    input_properties: HashMap<String, Property>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(default)]
    resources: HashMap<String, Resource>,
}

pub fn generate_files(path: &Path, result_path: &Path) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package: Package = serde_json::from_reader(reader)?;

    println!("{:#?}", package);

    Ok(())
}