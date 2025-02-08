#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OsPolicyAssignmentRolloutDisruptionBudget {
    /// Specifies a fixed value.
    #[builder(into, default)]
    #[serde(rename = "fixed")]
    pub r#fixed: Box<Option<i32>>,
    /// Specifies the relative value defined as a percentage,
    /// which will be multiplied by a reference value.
    /// 
    /// --------------------------------------------------------------------------------
    #[builder(into, default)]
    #[serde(rename = "percent")]
    pub r#percent: Box<Option<i32>>,
}
