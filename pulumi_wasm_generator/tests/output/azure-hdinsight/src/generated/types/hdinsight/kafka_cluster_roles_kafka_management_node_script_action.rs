#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KafkaClusterRolesKafkaManagementNodeScriptAction {
    /// The name of the script action.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The parameters for the script provided.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<String>>,
    /// The URI to the script.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}