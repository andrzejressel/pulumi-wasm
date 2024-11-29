#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerRuleFixedResponse {
    /// The value of the HTTP context-type header for this fixed response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// The value of the HTTP location header for this fixed response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The text used as the html body for this fixed response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "messageBody")]
    pub r#message_body: Box<Option<String>>,
    /// The HTTP status code used for this fixed response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
