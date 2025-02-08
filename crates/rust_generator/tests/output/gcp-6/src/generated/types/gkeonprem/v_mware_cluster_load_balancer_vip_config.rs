#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterLoadBalancerVipConfig {
    /// The VIP which you previously set aside for the Kubernetes API of this cluster.
    #[builder(into, default)]
    #[serde(rename = "controlPlaneVip")]
    pub r#control_plane_vip: Box<Option<String>>,
    /// The VIP which you previously set aside for ingress traffic into this cluster.
    /// 
    /// <a name="nested_f5_config"></a>The `f5_config` block supports:
    #[builder(into, default)]
    #[serde(rename = "ingressVip")]
    pub r#ingress_vip: Box<Option<String>>,
}
