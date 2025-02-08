#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationGatewayUrlPathMapPathRule {
    /// The ID of the associated Backend Address Pool.
    #[builder(into, default)]
    #[serde(rename = "backendAddressPoolId")]
    pub r#backend_address_pool_id: Box<Option<String>>,
    /// The Name of the Backend Address Pool to use for this Path Rule. Cannot be set if `redirect_configuration_name` is set.
    #[builder(into, default)]
    #[serde(rename = "backendAddressPoolName")]
    pub r#backend_address_pool_name: Box<Option<String>>,
    /// The ID of the associated Backend HTTP Settings Configuration.
    #[builder(into, default)]
    #[serde(rename = "backendHttpSettingsId")]
    pub r#backend_http_settings_id: Box<Option<String>>,
    /// The Name of the Backend HTTP Settings Collection to use for this Path Rule. Cannot be set if `redirect_configuration_name` is set.
    #[builder(into, default)]
    #[serde(rename = "backendHttpSettingsName")]
    pub r#backend_http_settings_name: Box<Option<String>>,
    /// The ID of the Web Application Firewall Policy which should be used as an HTTP Listener.
    #[builder(into, default)]
    #[serde(rename = "firewallPolicyId")]
    pub r#firewall_policy_id: Box<Option<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Name of the Path Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A list of Paths used in this Path Rule.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<String>>,
    /// The ID of the associated Redirect Configuration.
    #[builder(into, default)]
    #[serde(rename = "redirectConfigurationId")]
    pub r#redirect_configuration_id: Box<Option<String>>,
    /// The Name of a Redirect Configuration to use for this Path Rule. Cannot be set if `backend_address_pool_name` or `backend_http_settings_name` is set.
    #[builder(into, default)]
    #[serde(rename = "redirectConfigurationName")]
    pub r#redirect_configuration_name: Box<Option<String>>,
    /// The ID of the associated Rewrite Rule Set.
    #[builder(into, default)]
    #[serde(rename = "rewriteRuleSetId")]
    pub r#rewrite_rule_set_id: Box<Option<String>>,
    /// The Name of the Rewrite Rule Set which should be used for this URL Path Map. Only valid for v2 SKUs.
    #[builder(into, default)]
    #[serde(rename = "rewriteRuleSetName")]
    pub r#rewrite_rule_set_name: Box<Option<String>>,
}
