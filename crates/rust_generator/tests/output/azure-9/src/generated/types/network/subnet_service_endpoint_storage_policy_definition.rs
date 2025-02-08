#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SubnetServiceEndpointStoragePolicyDefinition {
    /// The description of this Subnet Service Endpoint Storage Policy Definition.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The name which should be used for this Subnet Service Endpoint Storage Policy Definition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of service resources. Valid values are `Microsoft.Storage` or `Global`. When the `service_resources` property contains resource IDs, this property must be `Microsoft.Storage`. When the `service_resources` property contains Aliases, this property must be `Global`. Defaults to `Microsoft.Storage`.
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    /// Specifies a list of resources or aliases that this Subnet Service Endpoint Storage Policy Definition applies to.
    /// 
    /// > **NOTE** The `service_resources` property must contain either Aliases or Resource IDs, but not both.
    #[builder(into)]
    #[serde(rename = "serviceResources")]
    pub r#service_resources: Box<Vec<String>>,
}
