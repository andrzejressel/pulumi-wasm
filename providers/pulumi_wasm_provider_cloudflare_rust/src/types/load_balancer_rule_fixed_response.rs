#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoadBalancerRuleFixedResponse {
    /// The value of the HTTP context-type header for this fixed response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// The value of the HTTP location header for this fixed response.
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The text used as the html body for this fixed response.
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    /// The HTTP status code used for this fixed response.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
