#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailSensitiveInformationPolicyConfig {
    /// List of entities. See PII Entities Config for more information.
    #[builder(into, default)]
    #[serde(rename = "piiEntitiesConfigs")]
    pub r#pii_entities_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigPiiEntitiesConfig>>>,
    /// List of regex. See Regexes Config for more information.
    #[builder(into, default)]
    #[serde(rename = "regexesConfigs")]
    pub r#regexes_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigRegexesConfig>>>,
}