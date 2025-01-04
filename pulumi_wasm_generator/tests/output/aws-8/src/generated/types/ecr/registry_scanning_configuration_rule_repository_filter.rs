#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryScanningConfigurationRuleRepositoryFilter {
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Box<String>,
    #[builder(into)]
    #[serde(rename = "filterType")]
    pub r#filter_type: Box<String>,
}
