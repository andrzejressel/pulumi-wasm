#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RateLimitMatchRequest {
    /// HTTP Methods to match traffic on. Available values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`, `_ALL_`.
    #[builder(into, default)]
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<String>>>,
    /// HTTP schemes to match traffic on. Available values: `HTTP`, `HTTPS`, `_ALL_`.
    #[builder(into, default)]
    #[serde(rename = "schemes")]
    pub r#schemes: Box<Option<Vec<String>>>,
    /// The URL pattern to match comprised of the host and path, i.e. example.org/path. Wildcard are expanded to match applicable traffic, query strings are not matched. Use _ for all traffic to your zone.
    #[builder(into, default)]
    #[serde(rename = "urlPattern")]
    pub r#url_pattern: Box<Option<String>>,
}
