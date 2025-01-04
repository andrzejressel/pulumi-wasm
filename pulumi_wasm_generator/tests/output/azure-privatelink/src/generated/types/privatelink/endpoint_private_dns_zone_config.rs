#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointPrivateDnsZoneConfig {
    /// The ID of the Private DNS Zone Config.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the Name of the Private Endpoint. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A list of IP Addresses
    #[builder(into, default)]
    #[serde(rename = "privateDnsZoneId")]
    pub r#private_dns_zone_id: Box<Option<String>>,
    /// A `record_sets` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "recordSets")]
    pub r#record_sets: Box<Option<Vec<super::super::types::privatelink::EndpointPrivateDnsZoneConfigRecordSet>>>,
}
