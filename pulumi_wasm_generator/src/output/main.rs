use crate::model::Package;
use crate::output::wit;
use crate::utils::get_main_version;
use anyhow::Context;
use rinja::Template;
use serde::Serialize;

#[derive(Template)]
#[template(path = "main.rs.jinja")]
struct TemplateModel<'a> {
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
    pulumi_wasm_wit: String,
    pulumi_wasm_version: &'a str,
    provider_name: &'a str,
    provider_version: &'a str,
    provider_metadata: &'a str,
}

#[derive(Serialize, Debug)]
struct WasmProviderVersion {
    pub version: String,
    #[serde(rename = "pluginDownloadURL")]
    pub plugin_download_url: Option<String>,
}

pub(crate) fn generate(
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
    package: &Package,
) -> anyhow::Result<String> {
    let wit = wit::get_dependencies(&package.name)?;

    let provider = WasmProviderVersion {
        version: package.version.clone(),
        plugin_download_url: package.plugin_download_url.clone(),
    };
    let provider = serde_json::to_string(&provider)
        .with_context(|| format!("Failed to serialize provider [{:?}]", provider))?;

    let file = TemplateModel {
        functions,
        resources,
        types,
        constants,
        pulumi_wasm_wit: wit,
        pulumi_wasm_version: get_main_version(),
        provider_name: &package.name,
        provider_version: &package.version,
        provider_metadata: &provider,
    }
    .render()?;

    let syntax_tree = syn::parse_file(file.as_str())?;
    let formatted = prettyplease::unparse(&syntax_tree);

    Ok(formatted)
}
