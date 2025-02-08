#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetListenerDefaultActionFixedResponse {
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<String>,
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
