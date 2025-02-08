#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RouteRequestParameter {
    /// Request parameter key. This is a [request data mapping parameter](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-data-mapping.html#websocket-mapping-request-parameters).
    #[builder(into)]
    #[serde(rename = "requestParameterKey")]
    pub r#request_parameter_key: Box<String>,
    /// Boolean whether or not the parameter is required.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Box<bool>,
}
