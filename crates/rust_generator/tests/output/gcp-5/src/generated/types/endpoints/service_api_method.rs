#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceApiMethod {
    /// The simple name of the endpoint as described in the config.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The type URL for the request to this API.
    #[builder(into, default)]
    #[serde(rename = "requestType")]
    pub r#request_type: Box<Option<String>>,
    /// The type URL for the response from this API.
    #[builder(into, default)]
    #[serde(rename = "responseType")]
    pub r#response_type: Box<Option<String>>,
    /// `SYNTAX_PROTO2` or `SYNTAX_PROTO3`.
    #[builder(into, default)]
    #[serde(rename = "syntax")]
    pub r#syntax: Box<Option<String>>,
}
