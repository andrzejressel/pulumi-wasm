#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkersScriptHyperdriveConfigBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    /// The ID of the Hyperdrive config to use.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
