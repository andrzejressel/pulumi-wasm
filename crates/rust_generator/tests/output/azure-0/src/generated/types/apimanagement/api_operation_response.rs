#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApiOperationResponse {
    /// A description of the HTTP Response, which may include HTML tags.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// One or more `header` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::apimanagement::ApiOperationResponseHeader>>>,
    /// One or more `representation` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "representations")]
    pub r#representations: Box<Option<Vec<super::super::types::apimanagement::ApiOperationResponseRepresentation>>>,
    /// The HTTP Status Code.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
}
