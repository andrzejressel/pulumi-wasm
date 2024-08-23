#[derive(serde::Serialize)]
pub struct LoadBalancerRuleFixedResponse {
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
