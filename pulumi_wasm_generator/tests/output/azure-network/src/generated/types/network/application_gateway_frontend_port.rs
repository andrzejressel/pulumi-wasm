#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayFrontendPort {
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the Frontend Port.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The port used for this Frontend Port.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}