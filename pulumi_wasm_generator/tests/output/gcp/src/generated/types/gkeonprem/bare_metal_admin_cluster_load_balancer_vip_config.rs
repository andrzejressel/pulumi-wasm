#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterLoadBalancerVipConfig {
    /// The VIP which you previously set aside for the Kubernetes API of this Bare Metal Admin Cluster.
    #[builder(into)]
    #[serde(rename = "controlPlaneVip")]
    pub r#control_plane_vip: Box<String>,
}
