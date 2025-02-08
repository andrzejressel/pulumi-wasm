#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNetworkGatewayCustomRoute {
    /// A list of address blocks reserved for this virtual network in CIDR notation.
    #[builder(into)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Box<Vec<String>>,
}
