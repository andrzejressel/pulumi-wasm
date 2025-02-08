#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPolicyRule {
    /// An `actions` block as documented below.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::storage::GetPolicyRuleAction>>,
    /// Boolean to specify whether the rule is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A `filter` block as documented below.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Vec<super::super::types::storage::GetPolicyRuleFilter>>,
    /// The filter tag name used for tag based filtering for blob objects.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
