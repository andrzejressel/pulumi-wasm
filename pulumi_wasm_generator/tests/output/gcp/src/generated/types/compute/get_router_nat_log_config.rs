#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouterNatLogConfig {
    /// Indicates whether or not to export logs.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Box<bool>,
    /// Specifies the desired filtering of logs on this NAT. Possible values: ["ERRORS_ONLY", "TRANSLATIONS_ONLY", "ALL"]
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Box<String>,
}
