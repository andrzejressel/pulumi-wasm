#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HaVpnGatewayVpnInterface {
    /// The numeric ID of this VPN gateway interface.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<i32>>,
    /// URL of the interconnect attachment resource. When the value
    /// of this field is present, the VPN Gateway will be used for
    /// IPsec-encrypted Cloud Interconnect; all Egress or Ingress
    /// traffic for this VPN Gateway interface will go through the
    /// specified interconnect attachment resource.
    /// Not currently available publicly.
    #[builder(into, default)]
    #[serde(rename = "interconnectAttachment")]
    pub r#interconnect_attachment: Box<Option<String>>,
    /// (Output)
    /// The external IP address for this VPN gateway interface.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
}
