#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResourcesRequests {
    /// CPU requirement expressed in Kubernetes resource units.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<String>>,
    /// Memory requirement expressed in Kubernetes resource units.
    #[builder(into, default)]
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<String>>,
}
