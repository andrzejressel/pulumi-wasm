#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DetectorDatasources {
    /// Configures [Kubernetes protection](https://docs.aws.amazon.com/guardduty/latest/ug/kubernetes-protection.html).
    /// See Kubernetes and Kubernetes Audit Logs below for more details.
    #[builder(into, default)]
    #[serde(rename = "kubernetes")]
    pub r#kubernetes: Box<Option<super::super::types::guardduty::DetectorDatasourcesKubernetes>>,
    /// Configures [Malware Protection](https://docs.aws.amazon.com/guardduty/latest/ug/malware-protection.html).
    /// See Malware Protection, Scan EC2 instance with findings and EBS volumes below for more details.
    #[builder(into, default)]
    #[serde(rename = "malwareProtection")]
    pub r#malware_protection: Box<Option<super::super::types::guardduty::DetectorDatasourcesMalwareProtection>>,
    /// Configures [S3 protection](https://docs.aws.amazon.com/guardduty/latest/ug/s3-protection.html).
    /// See S3 Logs below for more details.
    #[builder(into, default)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Box<Option<super::super::types::guardduty::DetectorDatasourcesS3Logs>>,
}
