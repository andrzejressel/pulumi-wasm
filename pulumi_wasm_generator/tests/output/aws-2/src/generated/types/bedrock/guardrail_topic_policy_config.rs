#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailTopicPolicyConfig {
    /// List of topic configs in topic policy. See Topics Config for more information.
    #[builder(into, default)]
    #[serde(rename = "topicsConfigs")]
    pub r#topics_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailTopicPolicyConfigTopicsConfig>>>,
}
