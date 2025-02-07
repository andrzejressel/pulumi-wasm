#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayUrlPathMap {
    /// The ID of the Default Backend Address Pool.
    #[builder(into, default)]
    #[serde(rename = "defaultBackendAddressPoolId")]
    pub r#default_backend_address_pool_id: Box<Option<String>>,
    /// The Name of the Default Backend Address Pool which should be used for this URL Path Map. Cannot be set if `default_redirect_configuration_name` is set.
    #[builder(into, default)]
    #[serde(rename = "defaultBackendAddressPoolName")]
    pub r#default_backend_address_pool_name: Box<Option<String>>,
    /// The ID of the Default Backend HTTP Settings Collection.
    #[builder(into, default)]
    #[serde(rename = "defaultBackendHttpSettingsId")]
    pub r#default_backend_http_settings_id: Box<Option<String>>,
    /// The Name of the Default Backend HTTP Settings Collection which should be used for this URL Path Map. Cannot be set if `default_redirect_configuration_name` is set.
    #[builder(into, default)]
    #[serde(rename = "defaultBackendHttpSettingsName")]
    pub r#default_backend_http_settings_name: Box<Option<String>>,
    /// The ID of the Default Redirect Configuration.
    #[builder(into, default)]
    #[serde(rename = "defaultRedirectConfigurationId")]
    pub r#default_redirect_configuration_id: Box<Option<String>>,
    /// The Name of the Default Redirect Configuration which should be used for this URL Path Map. Cannot be set if either `default_backend_address_pool_name` or `default_backend_http_settings_name` is set.
    /// 
    /// > **NOTE:** Both `default_backend_address_pool_name` and `default_backend_http_settings_name` or `default_redirect_configuration_name` should be specified.
    #[builder(into, default)]
    #[serde(rename = "defaultRedirectConfigurationName")]
    pub r#default_redirect_configuration_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "defaultRewriteRuleSetId")]
    pub r#default_rewrite_rule_set_id: Box<Option<String>>,
    /// The Name of the Default Rewrite Rule Set which should be used for this URL Path Map. Only valid for v2 SKUs.
    #[builder(into, default)]
    #[serde(rename = "defaultRewriteRuleSetName")]
    pub r#default_rewrite_rule_set_name: Box<Option<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Name of the URL Path Map.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// One or more `path_rule` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "pathRules")]
    pub r#path_rules: Box<Vec<super::super::types::network::ApplicationGatewayUrlPathMapPathRule>>,
}
