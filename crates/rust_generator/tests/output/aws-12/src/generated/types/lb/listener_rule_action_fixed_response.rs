#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleActionFixedResponse {
    /// The content type. Valid values are `text/plain`, `text/css`, `text/html`, `application/javascript` and `application/json`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    /// The message body.
    #[builder(into, default)]
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    /// The HTTP response code. Valid values are `2XX`, `4XX`, or `5XX`.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<String>>,
}
