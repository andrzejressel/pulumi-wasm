#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetHealthCheckLogConfig {
    /// Indicates whether or not to export logs. This is false by default,
    /// which means no health check logging will be done.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Box<bool>,
}
