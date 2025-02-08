#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetExternalAccessRuleDestinationIpRange {
    /// The name of an 'ExternalAddress' resource.
    #[builder(into)]
    #[serde(rename = "externalAddress")]
    pub r#external_address: Box<String>,
    /// An IP address range in the CIDR format.
    #[builder(into)]
    #[serde(rename = "ipAddressRange")]
    pub r#ip_address_range: Box<String>,
}
