#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfidentialLedgerAzureadBasedServicePrincipal {
    /// The Ledger Role to grant this Certificate Security Principal.
    #[builder(into)]
    #[serde(rename = "ledgerRoleName")]
    pub r#ledger_role_name: Box<String>,
    /// The Principal ID of the AzureAD Service Principal.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// The Tenant ID for this AzureAD Service Principal.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
