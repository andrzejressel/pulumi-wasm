#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNodeGroupLaunchTemplate {
    /// The ID of the launch template.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the AutoScaling Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Kubernetes version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}