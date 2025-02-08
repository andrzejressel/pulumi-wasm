#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualNetworkGatewayConnectionCustomBgpAddresses {
    /// single IP address that is part of the `azure.network.VirtualNetworkGateway` ip_configuration (first one)
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<String>,
    /// single IP address that is part of the `azure.network.VirtualNetworkGateway` ip_configuration (second one)
    #[builder(into, default)]
    #[serde(rename = "secondary")]
    pub r#secondary: Box<Option<String>>,
}
