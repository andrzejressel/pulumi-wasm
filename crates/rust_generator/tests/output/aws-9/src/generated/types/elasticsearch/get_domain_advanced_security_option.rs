#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainAdvancedSecurityOption {
    /// Whether node to node encryption is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Whether the internal user database is enabled.
    #[builder(into)]
    #[serde(rename = "internalUserDatabaseEnabled")]
    pub r#internal_user_database_enabled: Box<bool>,
}
