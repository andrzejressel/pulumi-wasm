#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterControlPlane {
    /// Customizes the default API server args. Only a subset of
    /// customized flags are supported. Please refer to the API server
    /// documentation below to know the exact format:
    /// https://kubernetes.io/docs/reference/command-line-tools-reference/kube-apiserver/
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "apiServerArgs")]
    pub r#api_server_args: Box<Option<Vec<super::super::types::gkeonprem::BareMetalAdminClusterControlPlaneApiServerArg>>>,
    /// Configures the node pool running the control plane. If specified the corresponding NodePool will be created for the cluster's control plane. The NodePool will have the same name and namespace as the cluster.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "controlPlaneNodePoolConfig")]
    pub r#control_plane_node_pool_config: Box<super::super::types::gkeonprem::BareMetalAdminClusterControlPlaneControlPlaneNodePoolConfig>,
}
