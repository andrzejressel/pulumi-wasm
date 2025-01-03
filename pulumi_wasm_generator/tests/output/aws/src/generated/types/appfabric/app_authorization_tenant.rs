#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppAuthorizationTenant {
    /// The display name of the tenant.
    #[builder(into)]
    #[serde(rename = "tenantDisplayName")]
    pub r#tenant_display_name: Box<String>,
    /// The ID of the application tenant.
    #[builder(into)]
    #[serde(rename = "tenantIdentifier")]
    pub r#tenant_identifier: Box<String>,
}
