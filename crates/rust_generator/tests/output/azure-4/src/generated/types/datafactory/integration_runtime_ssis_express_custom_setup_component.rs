#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationRuntimeSsisExpressCustomSetupComponent {
    /// A `key_vault_secret_reference` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "keyVaultLicense")]
    pub r#key_vault_license: Box<Option<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupComponentKeyVaultLicense>>,
    /// The license used for the Component.
    #[builder(into, default)]
    #[serde(rename = "license")]
    pub r#license: Box<Option<String>>,
    /// The Component Name installed for the Azure-SSIS Integration Runtime.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
