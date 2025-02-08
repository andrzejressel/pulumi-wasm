#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigSecurityConfig {
    /// Kerberos related configuration.
    #[builder(into, default)]
    #[serde(rename = "kerberosConfig")]
    pub r#kerberos_config: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigSecurityConfigKerberosConfig>>,
}
