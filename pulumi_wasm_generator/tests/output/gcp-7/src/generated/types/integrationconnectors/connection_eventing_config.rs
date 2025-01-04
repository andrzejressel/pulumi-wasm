#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionEventingConfig {
    /// List containing additional auth configs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Box<Option<Vec<super::super::types::integrationconnectors::ConnectionEventingConfigAdditionalVariable>>>,
    /// authConfig for Eventing Configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authConfig")]
    pub r#auth_config: Box<Option<super::super::types::integrationconnectors::ConnectionEventingConfigAuthConfig>>,
    /// Enrichment Enabled.
    #[builder(into, default)]
    #[serde(rename = "enrichmentEnabled")]
    pub r#enrichment_enabled: Box<Option<bool>>,
    /// registrationDestinationConfig
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "registrationDestinationConfig")]
    pub r#registration_destination_config: Box<super::super::types::integrationconnectors::ConnectionEventingConfigRegistrationDestinationConfig>,
}
