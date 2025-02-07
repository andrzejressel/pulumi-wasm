#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerDefaultActionFixedResponse {
    /// Content type. Valid values are `text/plain`, `text/css`, `text/html`, `application/javascript` and `application/json`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    /// Message body.
    #[builder(into, default)]
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    /// HTTP response code. Valid values are `2XX`, `4XX`, or `5XX`.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<String>>,
}
