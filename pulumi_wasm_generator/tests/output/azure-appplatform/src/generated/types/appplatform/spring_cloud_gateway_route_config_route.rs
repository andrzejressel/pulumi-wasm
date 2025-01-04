#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudGatewayRouteConfigRoute {
    /// Specifies the classification tags which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into, default)]
    #[serde(rename = "classificationTags")]
    pub r#classification_tags: Box<Option<Vec<String>>>,
    /// Specifies the description which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response.
    #[builder(into, default)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Option<Vec<String>>>,
    /// Specifies the route processing order.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<i32>,
    /// Specifies a list of conditions to evaluate a route for each request. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
    #[builder(into, default)]
    #[serde(rename = "predicates")]
    pub r#predicates: Box<Option<Vec<String>>>,
    /// Should the sso validation be enabled?
    #[builder(into, default)]
    #[serde(rename = "ssoValidationEnabled")]
    pub r#sso_validation_enabled: Box<Option<bool>>,
    /// Specifies the title which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into, default)]
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
    /// Should pass currently-authenticated user's identity token to application service?
    #[builder(into, default)]
    #[serde(rename = "tokenRelay")]
    pub r#token_relay: Box<Option<bool>>,
    /// Specifies the full uri which will override `appName`.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
