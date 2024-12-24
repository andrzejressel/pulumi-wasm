use crate::output::wit;
use crate::utils::get_main_version;
use rinja::Template;

#[derive(Template)]
#[template(path = "main.rs.jinja")]
struct TemplateModel {
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
    pulumi_wasm_wit: String,
    pulumi_wasm_version: String,
}

pub(crate) fn generate(
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
) -> anyhow::Result<String> {
    let wit = wit::get_dependencies()?;

    let file = TemplateModel {
        functions,
        resources,
        types,
        constants,
        pulumi_wasm_wit: wit,
        pulumi_wasm_version: get_main_version(),
    }
    .render()?;

    let syntax_tree = syn::parse_file(file.as_str())?;
    let formatted = prettyplease::unparse(&syntax_tree);

    Ok(formatted)
}
