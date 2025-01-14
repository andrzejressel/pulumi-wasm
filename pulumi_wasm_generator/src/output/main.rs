use crate::output::wit;
use crate::utils::get_main_version;
use rinja::Template;
use crate::model::Package;

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
}

pub(crate) fn generate(
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
    package: &Package,
) -> anyhow::Result<String> {
    let wit = wit::get_dependencies(&package.name)?;

    let file = TemplateModel {
        functions,
        resources,
        types,
        constants,
        pulumi_wasm_wit: wit,
        pulumi_wasm_version: get_main_version(),
        provider_name: &package.name,
        provider_version: &package.version,
    }
    .render()?;

    let syntax_tree = syn::parse_file(file.as_str())?;
    let formatted = prettyplease::unparse(&syntax_tree);

    Ok(formatted)
}
