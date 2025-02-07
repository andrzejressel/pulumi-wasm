#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointPrivateDnsZoneGroup {
    /// The ID of the Private DNS Zone Config.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the Name of the Private DNS Zone Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the list of Private DNS Zones to include within the `private_dns_zone_group`.
    #[builder(into)]
    #[serde(rename = "privateDnsZoneIds")]
    pub r#private_dns_zone_ids: Box<Vec<String>>,
}
