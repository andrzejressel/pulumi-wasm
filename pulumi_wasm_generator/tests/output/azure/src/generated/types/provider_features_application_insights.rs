#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderFeaturesApplicationInsights {
    #[builder(into, default)]
    #[serde(rename = "disableGeneratedRule")]
    pub r#disable_generated_rule: Box<Option<bool>>,
}
