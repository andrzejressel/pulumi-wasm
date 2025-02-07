#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "component")]
    pub r#component: Box<String>,
    /// Container resource requirements.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containerResources")]
    pub r#container_resources: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources>>,
    /// Pod affinity configuration.
    /// Possible values are: `AFFINITY_UNSPECIFIED`, `NO_AFFINITY`, `ANTI_AFFINITY`.
    #[builder(into, default)]
    #[serde(rename = "podAffinity")]
    pub r#pod_affinity: Box<Option<String>>,
    /// Pod tolerations of node taints.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "podTolerations")]
    pub r#pod_tolerations: Box<Option<Vec<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration>>>,
    /// Pod replica count.
    #[builder(into, default)]
    #[serde(rename = "replicaCount")]
    pub r#replica_count: Box<Option<i32>>,
}
