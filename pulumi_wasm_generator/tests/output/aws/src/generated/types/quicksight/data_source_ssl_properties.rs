#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceSslProperties {
    /// A Boolean option to control whether SSL should be disabled.
    #[builder(into)]
    #[serde(rename = "disableSsl")]
    pub r#disable_ssl: Box<bool>,
}