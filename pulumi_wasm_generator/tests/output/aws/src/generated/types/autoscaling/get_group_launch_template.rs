#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGroupLaunchTemplate {
    /// ID of the launch template.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Specify the exact name of the desired autoscaling group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Template version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
