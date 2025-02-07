#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationStoreReplica {
    /// The URL of the App Configuration Replica.
    #[builder(into, default)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<Option<String>>,
    /// The ID of the Access Key.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the supported Azure location where the replica exists.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Specifies the name of the replica.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
