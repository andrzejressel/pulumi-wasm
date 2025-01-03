#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigMultiTenant {
    /// Whether this project can have tenants or not.
    #[builder(into, default)]
    #[serde(rename = "allowTenants")]
    pub r#allow_tenants: Box<Option<bool>>,
    /// The default cloud parent org or folder that the tenant project should be created under.
    /// The parent resource name should be in the format of "/", such as "folders/123" or "organizations/456".
    /// If the value is not set, the tenant will be created under the same organization or folder as the agent project.
    #[builder(into, default)]
    #[serde(rename = "defaultTenantLocation")]
    pub r#default_tenant_location: Box<Option<String>>,
}
