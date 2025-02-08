#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWorkspaceEnhancedSecurityCompliance {
    /// Whether automatic cluster updates for this workspace is enabled.
    #[builder(into)]
    #[serde(rename = "automaticClusterUpdateEnabled")]
    pub r#automatic_cluster_update_enabled: Box<bool>,
    /// Whether compliance security profile for this workspace is enabled.
    #[builder(into)]
    #[serde(rename = "complianceSecurityProfileEnabled")]
    pub r#compliance_security_profile_enabled: Box<bool>,
    /// A list of standards enforced on this workspace.
    #[builder(into)]
    #[serde(rename = "complianceSecurityProfileStandards")]
    pub r#compliance_security_profile_standards: Box<Vec<String>>,
    /// Whether enhanced security monitoring for this workspace is enabled.
    #[builder(into)]
    #[serde(rename = "enhancedSecurityMonitoringEnabled")]
    pub r#enhanced_security_monitoring_enabled: Box<bool>,
}
