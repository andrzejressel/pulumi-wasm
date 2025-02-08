#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayRequestRoutingRule {
    /// The ID of the associated Backend Address Pool.
    #[builder(into, default)]
    #[serde(rename = "backendAddressPoolId")]
    pub r#backend_address_pool_id: Box<Option<String>>,
    /// The Name of the Backend Address Pool which should be used for this Routing Rule. Cannot be set if `redirect_configuration_name` is set.
    #[builder(into, default)]
    #[serde(rename = "backendAddressPoolName")]
    pub r#backend_address_pool_name: Box<Option<String>>,
    /// The ID of the associated Backend HTTP Settings Configuration.
    #[builder(into, default)]
    #[serde(rename = "backendHttpSettingsId")]
    pub r#backend_http_settings_id: Box<Option<String>>,
    /// The Name of the Backend HTTP Settings Collection which should be used for this Routing Rule. Cannot be set if `redirect_configuration_name` is set.
    #[builder(into, default)]
    #[serde(rename = "backendHttpSettingsName")]
    pub r#backend_http_settings_name: Box<Option<String>>,
    /// The ID of the associated HTTP Listener.
    #[builder(into, default)]
    #[serde(rename = "httpListenerId")]
    pub r#http_listener_id: Box<Option<String>>,
    /// The Name of the HTTP Listener which should be used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "httpListenerName")]
    pub r#http_listener_name: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Name of this Request Routing Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Rule evaluation order can be dictated by specifying an integer value from `1` to `20000` with `1` being the highest priority and `20000` being the lowest priority.
    /// 
    /// > **NOTE:** `priority` is required when `sku[0].tier` is set to `*_v2`.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// The ID of the associated Redirect Configuration.
    #[builder(into, default)]
    #[serde(rename = "redirectConfigurationId")]
    pub r#redirect_configuration_id: Box<Option<String>>,
    /// The Name of the Redirect Configuration which should be used for this Routing Rule. Cannot be set if either `backend_address_pool_name` or `backend_http_settings_name` is set.
    #[builder(into, default)]
    #[serde(rename = "redirectConfigurationName")]
    pub r#redirect_configuration_name: Box<Option<String>>,
    /// The ID of the associated Rewrite Rule Set.
    #[builder(into, default)]
    #[serde(rename = "rewriteRuleSetId")]
    pub r#rewrite_rule_set_id: Box<Option<String>>,
    /// The Name of the Rewrite Rule Set which should be used for this Routing Rule. Only valid for v2 SKUs.
    /// 
    /// > **NOTE:** `backend_address_pool_name`, `backend_http_settings_name`, `redirect_configuration_name`, and `rewrite_rule_set_name` are applicable only when `rule_type` is `Basic`.
    #[builder(into, default)]
    #[serde(rename = "rewriteRuleSetName")]
    pub r#rewrite_rule_set_name: Box<Option<String>>,
    /// The Type of Routing that should be used for this Rule. Possible values are `Basic` and `PathBasedRouting`.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: Box<String>,
    /// The ID of the associated URL Path Map.
    #[builder(into, default)]
    #[serde(rename = "urlPathMapId")]
    pub r#url_path_map_id: Box<Option<String>>,
    /// The Name of the URL Path Map which should be associated with this Routing Rule.
    #[builder(into, default)]
    #[serde(rename = "urlPathMapName")]
    pub r#url_path_map_name: Box<Option<String>>,
}
