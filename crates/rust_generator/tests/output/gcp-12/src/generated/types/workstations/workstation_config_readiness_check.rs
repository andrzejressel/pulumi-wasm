#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigReadinessCheck {
    /// Path to which the request should be sent.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Port to which the request should be sent.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
