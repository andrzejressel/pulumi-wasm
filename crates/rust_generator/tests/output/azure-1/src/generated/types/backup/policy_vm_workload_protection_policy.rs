#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyVmWorkloadProtectionPolicy {
    /// A `backup` block as defined below.
    #[builder(into)]
    #[serde(rename = "backup")]
    pub r#backup: Box<super::super::types::backup::PolicyVmWorkloadProtectionPolicyBackup>,
    /// The type of the VM Workload Backup Policy. Possible values are `Differential`, `Full`, `Incremental` and `Log`.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: Box<String>,
    /// A `retention_daily` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionDaily")]
    pub r#retention_daily: Box<Option<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionDaily>>,
    /// A `retention_monthly` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionMonthly")]
    pub r#retention_monthly: Box<Option<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionMonthly>>,
    /// A `retention_weekly` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionWeekly")]
    pub r#retention_weekly: Box<Option<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionWeekly>>,
    /// A `retention_yearly` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionYearly")]
    pub r#retention_yearly: Box<Option<super::super::types::backup::PolicyVmWorkloadProtectionPolicyRetentionYearly>>,
    /// A `simple_retention` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "simpleRetention")]
    pub r#simple_retention: Box<Option<super::super::types::backup::PolicyVmWorkloadProtectionPolicySimpleRetention>>,
}
