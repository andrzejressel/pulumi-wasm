#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationRuntimeSsisExpressCustomSetupCommandKey {
    /// A `key_vault_secret_reference` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "keyVaultPassword")]
    pub r#key_vault_password: Box<Option<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupCommandKeyKeyVaultPassword>>,
    /// The password for the target device.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The target computer or domain name.
    #[builder(into)]
    #[serde(rename = "targetName")]
    pub r#target_name: Box<String>,
    /// The username for the target device.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<String>,
}
