#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterLoggingConfigComponentConfig {
    /// Components of the logging configuration to be enabled.
    #[builder(into, default)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Box<Option<Vec<String>>>,
}
