#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApiOperationRequest {
    /// A description of the HTTP Request, which may include HTML tags.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// One or more `header` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::apimanagement::ApiOperationRequestHeader>>>,
    /// One or more `query_parameter` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Option<Vec<super::super::types::apimanagement::ApiOperationRequestQueryParameter>>>,
    /// One or more `representation` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "representations")]
    pub r#representations: Box<Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentation>>>,
}
