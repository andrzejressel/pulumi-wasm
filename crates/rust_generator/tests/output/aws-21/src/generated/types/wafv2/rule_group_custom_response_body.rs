#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleGroupCustomResponseBody {
    /// The payload of the custom response.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// The type of content in the payload that you are defining in the `content` argument. Valid values are `TEXT_PLAIN`, `TEXT_HTML`, or `APPLICATION_JSON`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
    /// A unique key identifying the custom response body. This is referenced by the `custom_response_body_key` argument in the Custom Response block.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
