#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTenantAccess {
    /// Should the access to the management API be enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Primary access key for the tenant access information contract.
    #[builder(into, default)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Box<Option<String>>,
    /// Secondary access key for the tenant access information contract.
    #[builder(into, default)]
    #[serde(rename = "secondaryKey")]
    pub r#secondary_key: Box<Option<String>>,
    /// The identifier for the tenant access information contract.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
}
