#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetManagedZonesManagedZone {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[builder(into)]
    #[serde(rename = "managedZoneId")]
    pub r#managed_zone_id: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "nameServers")]
    pub r#name_servers: Box<Vec<String>>,
    /// The ID of the project containing Google Cloud DNS zones. If this is not provided the default project will be used.
    #[builder(into, default)]
    #[serde(rename = "project")]
    pub r#project: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: Box<String>,
}
