#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayUrlPathMap {
    /// The ID of the Default Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "defaultBackendAddressPoolId")]
    pub r#default_backend_address_pool_id: Box<String>,
    /// The Name of the Default Backend Address Pool which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultBackendAddressPoolName")]
    pub r#default_backend_address_pool_name: Box<String>,
    /// The ID of the Default Backend HTTP Settings Collection.
    #[builder(into)]
    #[serde(rename = "defaultBackendHttpSettingsId")]
    pub r#default_backend_http_settings_id: Box<String>,
    /// The Name of the Default Backend HTTP Settings Collection which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultBackendHttpSettingsName")]
    pub r#default_backend_http_settings_name: Box<String>,
    /// The ID of the Default Redirect Configuration.
    #[builder(into)]
    #[serde(rename = "defaultRedirectConfigurationId")]
    pub r#default_redirect_configuration_id: Box<String>,
    /// The Name of the Default Redirect Configuration which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultRedirectConfigurationName")]
    pub r#default_redirect_configuration_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "defaultRewriteRuleSetId")]
    pub r#default_rewrite_rule_set_id: Box<String>,
    /// The Name of the Default Rewrite Rule Set which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultRewriteRuleSetName")]
    pub r#default_rewrite_rule_set_name: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `path_rule` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "pathRules")]
    pub r#path_rules: Box<Vec<super::super::types::network::GetApplicationGatewayUrlPathMapPathRule>>,
}
