#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorRuleActions {
    /// A `request_header_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaderActions")]
    pub r#request_header_actions: Box<Option<Vec<super::super::types::cdn::FrontdoorRuleActionsRequestHeaderAction>>>,
    /// A `response_header_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "responseHeaderActions")]
    pub r#response_header_actions: Box<Option<Vec<super::super::types::cdn::FrontdoorRuleActionsResponseHeaderAction>>>,
    /// A `route_configuration_override_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "routeConfigurationOverrideAction")]
    pub r#route_configuration_override_action: Box<Option<super::super::types::cdn::FrontdoorRuleActionsRouteConfigurationOverrideAction>>,
    /// A `url_redirect_action` block as defined below. You may **not** have a `url_redirect_action` **and** a `url_rewrite_action` defined in the same `actions` block.
    #[builder(into, default)]
    #[serde(rename = "urlRedirectAction")]
    pub r#url_redirect_action: Box<Option<super::super::types::cdn::FrontdoorRuleActionsUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below. You may **not** have a `url_rewrite_action` **and** a `url_redirect_action` defined in the same `actions` block.
    #[builder(into, default)]
    #[serde(rename = "urlRewriteAction")]
    pub r#url_rewrite_action: Box<Option<super::super::types::cdn::FrontdoorRuleActionsUrlRewriteAction>>,
}
