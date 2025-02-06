#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountNetworkRulesPrivateLinkAccessRule {
    /// The resource id of the resource access rule to be granted access.
    #[builder(into)]
    #[serde(rename = "endpointResourceId")]
    pub r#endpoint_resource_id: Box<String>,
    /// The tenant id of the resource of the resource access rule to be granted access. Defaults to the current tenant id.
    #[builder(into, default)]
    #[serde(rename = "endpointTenantId")]
    pub r#endpoint_tenant_id: Box<Option<String>>,
}
