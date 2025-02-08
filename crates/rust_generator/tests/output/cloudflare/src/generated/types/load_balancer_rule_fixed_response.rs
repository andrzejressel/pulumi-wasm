#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerRuleFixedResponse {
    /// The value of the HTTP context-type header for this fixed response.
    #[builder(into, default)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// The value of the HTTP location header for this fixed response.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The text used as the html body for this fixed response.
    #[builder(into, default)]
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    /// The HTTP status code used for this fixed response.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
