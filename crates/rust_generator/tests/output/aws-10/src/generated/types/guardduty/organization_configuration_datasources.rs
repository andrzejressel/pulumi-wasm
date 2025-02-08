#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OrganizationConfigurationDatasources {
    /// Enable Kubernetes Audit Logs Monitoring automatically for new member accounts.
    #[builder(into, default)]
    #[serde(rename = "kubernetes")]
    pub r#kubernetes: Box<Option<super::super::types::guardduty::OrganizationConfigurationDatasourcesKubernetes>>,
    /// Enable Malware Protection automatically for new member accounts.
    #[builder(into, default)]
    #[serde(rename = "malwareProtection")]
    pub r#malware_protection: Box<Option<super::super::types::guardduty::OrganizationConfigurationDatasourcesMalwareProtection>>,
    /// Enable S3 Protection automatically for new member accounts.
    #[builder(into, default)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Box<Option<super::super::types::guardduty::OrganizationConfigurationDatasourcesS3Logs>>,
}
