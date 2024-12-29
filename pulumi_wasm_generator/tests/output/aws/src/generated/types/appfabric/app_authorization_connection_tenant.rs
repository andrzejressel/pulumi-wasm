#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppAuthorizationConnectionTenant {
    #[builder(into)]
    #[serde(rename = "tenantDisplayName")]
    pub r#tenant_display_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "tenantIdentifier")]
    pub r#tenant_identifier: Box<String>,
}
