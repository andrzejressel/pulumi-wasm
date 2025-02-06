#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalAdminClusterLoadBalancer {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "manualLbConfig")]
    pub r#manual_lb_config: Box<Option<super::super::types::gkeonprem::BareMetalAdminClusterLoadBalancerManualLbConfig>>,
    /// Specifies the load balancer ports.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "portConfig")]
    pub r#port_config: Box<super::super::types::gkeonprem::BareMetalAdminClusterLoadBalancerPortConfig>,
    /// Specified the Bare Metal Load Balancer Config
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vipConfig")]
    pub r#vip_config: Box<super::super::types::gkeonprem::BareMetalAdminClusterLoadBalancerVipConfig>,
}
