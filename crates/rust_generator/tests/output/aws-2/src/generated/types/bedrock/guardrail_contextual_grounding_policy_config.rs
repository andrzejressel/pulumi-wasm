#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailContextualGroundingPolicyConfig {
    /// List of contextual grounding filter configs. See Contextual Grounding Filters Config for more information.
    #[builder(into, default)]
    #[serde(rename = "filtersConfigs")]
    pub r#filters_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailContextualGroundingPolicyConfigFiltersConfig>>>,
}
