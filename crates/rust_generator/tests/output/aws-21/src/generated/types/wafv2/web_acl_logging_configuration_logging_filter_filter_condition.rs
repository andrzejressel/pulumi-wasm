#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclLoggingConfigurationLoggingFilterFilterCondition {
    /// Configuration for a single action condition. See Action Condition below for more details.
    #[builder(into, default)]
    #[serde(rename = "actionCondition")]
    pub r#action_condition: Box<Option<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterConditionActionCondition>>,
    /// Condition for a single label name. See Label Name Condition below for more details.
    #[builder(into, default)]
    #[serde(rename = "labelNameCondition")]
    pub r#label_name_condition: Box<Option<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterConditionLabelNameCondition>>,
}
