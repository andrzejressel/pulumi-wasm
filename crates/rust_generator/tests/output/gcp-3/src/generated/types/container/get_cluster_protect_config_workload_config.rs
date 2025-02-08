#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterProtectConfigWorkloadConfig {
    /// Sets which mode of auditing should be used for the cluster's workloads. Accepted values are DISABLED, BASIC.
    #[builder(into)]
    #[serde(rename = "auditMode")]
    pub r#audit_mode: Box<String>,
}
