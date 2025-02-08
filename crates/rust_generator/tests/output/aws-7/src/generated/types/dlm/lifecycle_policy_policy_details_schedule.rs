#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailsSchedule {
    #[builder(into, default)]
    #[serde(rename = "copyTags")]
    pub r#copy_tags: Box<Option<bool>>,
    /// See the `create_rule` block. Max of 1 per schedule.
    #[builder(into)]
    #[serde(rename = "createRule")]
    pub r#create_rule: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCreateRule>,
    /// See the `cross_region_copy_rule` block. Max of 3 per schedule.
    #[builder(into, default)]
    #[serde(rename = "crossRegionCopyRules")]
    pub r#cross_region_copy_rules: Box<Option<Vec<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule>>>,
    #[builder(into, default)]
    #[serde(rename = "deprecateRule")]
    pub r#deprecate_rule: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleDeprecateRule>>,
    /// See the `fast_restore_rule` block. Max of 1 per schedule.
    #[builder(into, default)]
    #[serde(rename = "fastRestoreRule")]
    pub r#fast_restore_rule: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleFastRestoreRule>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleRetainRule>,
    /// See the `share_rule` block. Max of 1 per schedule.
    #[builder(into, default)]
    #[serde(rename = "shareRule")]
    pub r#share_rule: Box<Option<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleShareRule>>,
    /// A map of tag keys and their values. DLM lifecycle policies will already tag the snapshot with the tags on the volume. This configuration adds extra tags on top of these.
    #[builder(into, default)]
    #[serde(rename = "tagsToAdd")]
    pub r#tags_to_add: Box<Option<std::collections::HashMap<String, String>>>,
    /// A map of tag keys and variable values, where the values are determined when the policy is executed. Only `$(instance-id)` or `$(timestamp)` are valid values. Can only be used when `resource_types` is `INSTANCE`.
    #[builder(into, default)]
    #[serde(rename = "variableTags")]
    pub r#variable_tags: Box<Option<std::collections::HashMap<String, String>>>,
}
