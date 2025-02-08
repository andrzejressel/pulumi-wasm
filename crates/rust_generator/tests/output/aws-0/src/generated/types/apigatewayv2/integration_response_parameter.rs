#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationResponseParameter {
    /// Key-value map. The key of this map identifies the location of the request parameter to change, and how to change it. The corresponding value specifies the new data for the parameter.
    /// See the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html) for details.
    #[builder(into)]
    #[serde(rename = "mappings")]
    pub r#mappings: Box<std::collections::HashMap<String, String>>,
    /// HTTP status code in the range 200-599.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
