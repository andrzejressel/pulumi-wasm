#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationConfigurationDatasourcesKubernetes {
    /// Enable Kubernetes Audit Logs Monitoring automatically for new member accounts. [Kubernetes protection](https://docs.aws.amazon.com/guardduty/latest/ug/kubernetes-protection.html).
    /// See Kubernetes Audit Logs below for more details.
    #[builder(into)]
    #[serde(rename = "auditLogs")]
    pub r#audit_logs: Box<super::super::types::guardduty::OrganizationConfigurationDatasourcesKubernetesAuditLogs>,
}