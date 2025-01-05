use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn find_modules(provider_name: &str) -> Result<Vec<String>> {
    let file_content = fs::read_to_string(format!("providers/{provider_name}.json"))?;
    let provider: Provider = serde_json::from_str(&file_content)?;

    let mut resources_modules: HashSet<&str> = provider
        .resources
        .keys()
        .flat_map(|k| extract_module(k))
        .collect();
    let functions_modules: HashSet<&str> = provider
        .functions
        .keys()
        .flat_map(|k| extract_module(k))
        .collect();
    let types_modules: HashSet<&str> = provider
        .types
        .keys()
        .flat_map(|k| extract_module(k))
        .collect();

    resources_modules.extend(functions_modules);
    resources_modules.extend(types_modules);
    let mut resources_modules = resources_modules.iter().collect::<Vec<_>>();
    resources_modules.sort();

    Ok(resources_modules.iter().map(|s| s.to_string()).collect())
}

fn extract_module(input: &str) -> Option<&str> {
    input
        .split(':')
        .nth(1)
        .map(|s| s.split('/').next())
        .flatten()
}

#[derive(Deserialize)]
struct Provider {
    types: HashMap<String, Empty>,
    resources: HashMap<String, Empty>,
    functions: HashMap<String, Empty>,
}

#[derive(Deserialize)]
struct Empty {}
