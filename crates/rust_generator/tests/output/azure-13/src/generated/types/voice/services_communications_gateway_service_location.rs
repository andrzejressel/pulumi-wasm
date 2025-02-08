#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServicesCommunicationsGatewayServiceLocation {
    /// Specifies the allowed source IP address or CIDR ranges for media.
    #[builder(into, default)]
    #[serde(rename = "allowedMediaSourceAddressPrefixes")]
    pub r#allowed_media_source_address_prefixes: Box<Option<Vec<String>>>,
    /// Specifies the allowed source IP address or CIDR ranges for signaling.
    #[builder(into, default)]
    #[serde(rename = "allowedSignalingSourceAddressPrefixes")]
    pub r#allowed_signaling_source_address_prefixes: Box<Option<Vec<String>>>,
    /// IP address to use to contact the ESRP from this region.
    /// 
    /// !> **NOTE:** The `esrp_addresses` must be specified for each `service_location` when the`e911_type` is set to `DirectToEsrp`.  The `esrp_addresses` must not be specified for each `service_location` when the`e911_type` is set to `Standard`.
    #[builder(into, default)]
    #[serde(rename = "esrpAddresses")]
    pub r#esrp_addresses: Box<Option<Vec<String>>>,
    /// Specifies the region in which the resources needed for Teams Calling will be deployed.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// IP address to use to contact the operator network from this region.
    #[builder(into)]
    #[serde(rename = "operatorAddresses")]
    pub r#operator_addresses: Box<Vec<String>>,
}
