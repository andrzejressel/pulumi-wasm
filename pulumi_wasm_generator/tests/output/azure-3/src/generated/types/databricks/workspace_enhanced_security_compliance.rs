#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkspaceEnhancedSecurityCompliance {
    /// Enables automatic cluster updates for this workspace. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "automaticClusterUpdateEnabled")]
    pub r#automatic_cluster_update_enabled: Box<Option<bool>>,
    /// Enables compliance security profile for this workspace. Defaults to `false`.
    /// 
    /// > **Note:** Changing the value of `compliance_security_profile_enabled` from `true` to `false` forces a replacement of the Databricks workspace.
    /// 
    /// > **Note:** The attributes `automatic_cluster_update_enabled` and `enhanced_security_monitoring_enabled` must be set to `true` in order to set `compliance_security_profile_enabled` to `true`.
    #[builder(into, default)]
    #[serde(rename = "complianceSecurityProfileEnabled")]
    pub r#compliance_security_profile_enabled: Box<Option<bool>>,
    /// A list of standards to enforce on this workspace. Possible values include `HIPAA` and `PCI_DSS`.
    /// 
    /// > **Note:** `compliance_security_profile_enabled` must be set to `true` in order to use `compliance_security_profile_standards`.
    /// 
    /// > **Note:** Removing a standard from the `compliance_security_profile_standards` list forces a replacement of the Databricks workspace.
    #[builder(into, default)]
    #[serde(rename = "complianceSecurityProfileStandards")]
    pub r#compliance_security_profile_standards: Box<Option<Vec<String>>>,
    /// Enables enhanced security monitoring for this workspace. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enhancedSecurityMonitoringEnabled")]
    pub r#enhanced_security_monitoring_enabled: Box<Option<bool>>,
}
