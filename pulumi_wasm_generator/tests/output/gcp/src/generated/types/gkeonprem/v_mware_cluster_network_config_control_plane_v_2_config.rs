#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterNetworkConfigControlPlaneV2Config {
    /// Static IP addresses for the control plane nodes.
    #[builder(into, default)]
    #[serde(rename = "controlPlaneIpBlock")]
    pub r#control_plane_ip_block: Box<Option<super::super::types::gkeonprem::VMwareClusterNetworkConfigControlPlaneV2ConfigControlPlaneIpBlock>>,
}
