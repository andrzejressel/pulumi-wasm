#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetUpdateRunManagedClusterUpdateUpgrade {
    /// Specifies the Kubernetes version to upgrade the member clusters to. This is required if `type` is set to `Full`.
    #[builder(into, default)]
    #[serde(rename = "kubernetesVersion")]
    pub r#kubernetes_version: Box<Option<String>>,
    /// Specifies the type of upgrade to perform. Possible values are `Full` and `NodeImageOnly`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
