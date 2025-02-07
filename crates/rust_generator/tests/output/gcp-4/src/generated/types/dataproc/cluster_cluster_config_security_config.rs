#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigSecurityConfig {
    /// Kerberos Configuration
    #[builder(into)]
    #[serde(rename = "kerberosConfig")]
    pub r#kerberos_config: Box<super::super::types::dataproc::ClusterClusterConfigSecurityConfigKerberosConfig>,
}
