#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventTargetHttpTarget {
    /// Enables you to specify HTTP headers to add to the request.
    #[builder(into, default)]
    #[serde(rename = "headerParameters")]
    pub r#header_parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// The list of values that correspond sequentially to any path variables in your endpoint ARN (for example `arn:aws:execute-api:us-east-1:123456:myapi/*/POST/pets/*`).
    #[builder(into, default)]
    #[serde(rename = "pathParameterValues")]
    pub r#path_parameter_values: Box<Option<Vec<String>>>,
    /// Represents keys/values of query string parameters that are appended to the invoked endpoint.
    #[builder(into, default)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Box<Option<std::collections::HashMap<String, String>>>,
}
