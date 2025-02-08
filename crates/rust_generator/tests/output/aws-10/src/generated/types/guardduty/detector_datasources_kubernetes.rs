#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DetectorDatasourcesKubernetes {
    /// Configures Kubernetes audit logs as a data source for [Kubernetes protection](https://docs.aws.amazon.com/guardduty/latest/ug/kubernetes-protection.html).
    /// See Kubernetes Audit Logs below for more details.
    #[builder(into)]
    #[serde(rename = "auditLogs")]
    pub r#audit_logs: Box<super::super::types::guardduty::DetectorDatasourcesKubernetesAuditLogs>,
}
