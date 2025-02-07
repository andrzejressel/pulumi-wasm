#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryGeoreplication {
    /// A location where the container registry should be geo-replicated.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Whether regional endpoint is enabled for this Container Registry?
    #[builder(into, default)]
    #[serde(rename = "regionalEndpointEnabled")]
    pub r#regional_endpoint_enabled: Box<Option<bool>>,
    /// A mapping of tags to assign to this replication location.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Whether zone redundancy is enabled for this replication location? Defaults to `false`.
    /// 
    /// > **NOTE:** Changing the `zone_redundancy_enabled` forces the a underlying replication to be created.
    #[builder(into, default)]
    #[serde(rename = "zoneRedundancyEnabled")]
    pub r#zone_redundancy_enabled: Box<Option<bool>>,
}
