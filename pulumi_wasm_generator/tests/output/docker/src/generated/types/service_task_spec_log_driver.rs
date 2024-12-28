#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpecLogDriver {
    /// The logging driver to use
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The options for the logging driver
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
}
