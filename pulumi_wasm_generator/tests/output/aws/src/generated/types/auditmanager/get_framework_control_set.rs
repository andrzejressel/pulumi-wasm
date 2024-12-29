#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFrameworkControlSet {
    #[builder(into, default)]
    #[serde(rename = "controls")]
    pub r#controls: Box<Option<Vec<super::super::types::auditmanager::GetFrameworkControlSetControl>>>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the framework.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
