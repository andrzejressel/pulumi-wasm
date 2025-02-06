#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SoftwareUpdateConfigurationPostTask {
    /// Specifies a map of parameters for the task.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of the runbook for the post task.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
}
