#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesSharedStorage {
    /// The full name of the Shared Storage Account. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<Option<String>>,
    /// The full name of Private Endpoint for the Shared Storage Account. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "privateEndpointName")]
    pub r#private_endpoint_name: Box<Option<String>>,
}
