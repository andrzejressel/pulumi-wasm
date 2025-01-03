#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentLastDeploymentFailureReason {
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Box<String>,
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<String>,
}
