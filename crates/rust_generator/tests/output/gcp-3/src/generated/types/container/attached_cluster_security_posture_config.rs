#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AttachedClusterSecurityPostureConfig {
    /// Sets the mode of the Kubernetes security posture API's workload vulnerability scanning.
    /// Possible values are: `VULNERABILITY_DISABLED`, `VULNERABILITY_ENTERPRISE`.
    #[builder(into)]
    #[serde(rename = "vulnerabilityMode")]
    pub r#vulnerability_mode: Box<String>,
}
