#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterLoadBalancerVipConfig {
    /// The VIP which you previously set aside for the Kubernetes API of this Bare Metal User Cluster.
    #[builder(into)]
    #[serde(rename = "controlPlaneVip")]
    pub r#control_plane_vip: Box<String>,
    /// The VIP which you previously set aside for ingress traffic into this Bare Metal User Cluster.
    #[builder(into)]
    #[serde(rename = "ingressVip")]
    pub r#ingress_vip: Box<String>,
}
