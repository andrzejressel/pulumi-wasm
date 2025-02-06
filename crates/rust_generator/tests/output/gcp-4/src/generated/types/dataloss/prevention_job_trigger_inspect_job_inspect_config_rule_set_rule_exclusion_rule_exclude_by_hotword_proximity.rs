#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordProximity {
    /// Number of characters after the finding to consider. Either this or window_before must be specified
    #[builder(into, default)]
    #[serde(rename = "windowAfter")]
    pub r#window_after: Box<Option<i32>>,
    /// Number of characters before the finding to consider. Either this or window_after must be specified
    #[builder(into, default)]
    #[serde(rename = "windowBefore")]
    pub r#window_before: Box<Option<i32>>,
}
