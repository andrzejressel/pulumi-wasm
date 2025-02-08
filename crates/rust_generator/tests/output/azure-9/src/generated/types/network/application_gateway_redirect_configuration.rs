#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationGatewayRedirectConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Whether to include the path in the redirected URL. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "includePath")]
    pub r#include_path: Box<Option<bool>>,
    /// Whether to include the query string in the redirected URL. Default to `false`
    #[builder(into, default)]
    #[serde(rename = "includeQueryString")]
    pub r#include_query_string: Box<Option<bool>>,
    /// Unique name of the redirect configuration block
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of redirect. Possible values are `Permanent`, `Temporary`, `Found` and `SeeOther`
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "targetListenerId")]
    pub r#target_listener_id: Box<Option<String>>,
    /// The name of the listener to redirect to. Cannot be set if `target_url` is set.
    #[builder(into, default)]
    #[serde(rename = "targetListenerName")]
    pub r#target_listener_name: Box<Option<String>>,
    /// The URL to redirect the request to. Cannot be set if `target_listener_name` is set.
    #[builder(into, default)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<Option<String>>,
}
