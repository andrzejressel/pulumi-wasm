#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailsScheduleDeprecateRule {
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "intervalUnit")]
    pub r#interval_unit: Box<Option<String>>,
}
