#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWorkspaceStorageAccountIdentity {
    /// The principal UUID for the internal databricks storage account needed to provide access to the workspace for enabling Customer Managed Keys.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The UUID of the tenant where the internal databricks storage account was created.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
    /// The type of the internal databricks storage account.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}