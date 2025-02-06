#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttpRouteActionRewriteHostname {
    /// Default target host name to write to. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "defaultTargetHostname")]
    pub r#default_target_hostname: Box<String>,
}
