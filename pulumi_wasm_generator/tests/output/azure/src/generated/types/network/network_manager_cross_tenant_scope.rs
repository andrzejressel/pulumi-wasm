#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkManagerCrossTenantScope {
    /// List of management groups.
    #[builder(into, default)]
    #[serde(rename = "managementGroups")]
    pub r#management_groups: Box<Option<Vec<String>>>,
    /// List of subscriptions.
    #[builder(into, default)]
    #[serde(rename = "subscriptions")]
    pub r#subscriptions: Box<Option<Vec<String>>>,
    /// Tenant ID.
    #[builder(into, default)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<Option<String>>,
}