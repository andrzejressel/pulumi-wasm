#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayUrlPathMapPathRule {
    /// The ID of the associated Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "backendAddressPoolId")]
    pub r#backend_address_pool_id: Box<String>,
    /// The Name of the Backend Address Pool which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "backendAddressPoolName")]
    pub r#backend_address_pool_name: Box<String>,
    /// The ID of the associated Backend HTTP Settings Configuration.
    #[builder(into)]
    #[serde(rename = "backendHttpSettingsId")]
    pub r#backend_http_settings_id: Box<String>,
    /// The Name of the Backend HTTP Settings Collection which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "backendHttpSettingsName")]
    pub r#backend_http_settings_name: Box<String>,
    /// The ID of the Web Application Firewall Policy which is used as an HTTP Listener for this Path Rule.
    #[builder(into)]
    #[serde(rename = "firewallPolicyId")]
    pub r#firewall_policy_id: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A list of Paths used in this Path Rule.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<String>>,
    /// The ID of the associated Redirect Configuration.
    #[builder(into)]
    #[serde(rename = "redirectConfigurationId")]
    pub r#redirect_configuration_id: Box<String>,
    /// The Name of the Redirect Configuration which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "redirectConfigurationName")]
    pub r#redirect_configuration_name: Box<String>,
    /// The ID of the associated Rewrite Rule Set.
    #[builder(into)]
    #[serde(rename = "rewriteRuleSetId")]
    pub r#rewrite_rule_set_id: Box<String>,
    /// The Name of the Rewrite Rule Set which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "rewriteRuleSetName")]
    pub r#rewrite_rule_set_name: Box<String>,
}