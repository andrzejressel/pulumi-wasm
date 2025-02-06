#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
    /// The name of the component. One of `admission` `audit` or `mutation`
    #[builder(into)]
    #[serde(rename = "componentName")]
    pub r#component_name: Box<String>,
    /// Container resource requirements.
    #[builder(into, default)]
    #[serde(rename = "containerResources")]
    pub r#container_resources: Box<Option<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources>>,
    /// Pod affinity configuration. Possible values: AFFINITY_UNSPECIFIED, NO_AFFINITY, ANTI_AFFINITY
    #[builder(into, default)]
    #[serde(rename = "podAffinity")]
    pub r#pod_affinity: Box<Option<String>>,
    /// Pod tolerations of node taints.
    #[builder(into, default)]
    #[serde(rename = "podTolerations")]
    pub r#pod_tolerations: Box<Option<Vec<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration>>>,
    /// Pod replica count.
    #[builder(into, default)]
    #[serde(rename = "replicaCount")]
    pub r#replica_count: Box<Option<i32>>,
}
