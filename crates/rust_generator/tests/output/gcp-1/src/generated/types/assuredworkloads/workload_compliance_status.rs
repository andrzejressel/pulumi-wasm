#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadComplianceStatus {
    /// Number of current orgPolicy violations which are acknowledged.
    #[builder(into, default)]
    #[serde(rename = "acknowledgedViolationCounts")]
    pub r#acknowledged_violation_counts: Box<Option<Vec<i32>>>,
    /// Number of current orgPolicy violations which are not acknowledged.
    #[builder(into, default)]
    #[serde(rename = "activeViolationCounts")]
    pub r#active_violation_counts: Box<Option<Vec<i32>>>,
}
