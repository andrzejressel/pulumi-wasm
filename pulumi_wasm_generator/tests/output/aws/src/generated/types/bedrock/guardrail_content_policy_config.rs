#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailContentPolicyConfig {
    /// Set of content filter configs in content policy.
    /// See Filters Config for more information.
    #[builder(into, default)]
    #[serde(rename = "filtersConfigs")]
    pub r#filters_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailContentPolicyConfigFiltersConfig>>>,
}
