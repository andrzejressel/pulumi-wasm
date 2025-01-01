#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LedgerAzureadBasedServicePrincipal {
    /// Specifies the Ledger Role to grant this AzureAD Service Principal. Possible values are `Administrator`, `Contributor` and `Reader`.
    #[builder(into)]
    #[serde(rename = "ledgerRoleName")]
    pub r#ledger_role_name: Box<String>,
    /// Specifies the Principal ID of the AzureAD Service Principal.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// Specifies the Tenant ID for this AzureAD Service Principal.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
