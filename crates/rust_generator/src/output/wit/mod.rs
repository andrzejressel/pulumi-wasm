use crate::utils::get_main_version;
use handlebars::Handlebars;

static DEPENDENCIES: &str = include_str!("dependencies.handlebars");

pub(crate) fn get_dependencies(provider_name: &str) -> anyhow::Result<String> {
    let mut data = std::collections::BTreeMap::new();
    data.insert("pulumi_gestalt_version", get_main_version());
    data.insert("provider_name", provider_name);

    let reg = Handlebars::new();
    let output = reg.render_template(DEPENDENCIES, &data)?;

    Ok(output)
}
