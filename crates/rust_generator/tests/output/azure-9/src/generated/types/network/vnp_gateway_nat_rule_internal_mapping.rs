#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VnpGatewayNatRuleInternalMapping {
    /// The string CIDR representing the address space for the VPN Gateway Nat Rule internal mapping.
    #[builder(into)]
    #[serde(rename = "addressSpace")]
    pub r#address_space: Box<String>,
    /// The single port range for the VPN Gateway Nat Rule internal mapping.
    #[builder(into, default)]
    #[serde(rename = "portRange")]
    pub r#port_range: Box<Option<String>>,
}
