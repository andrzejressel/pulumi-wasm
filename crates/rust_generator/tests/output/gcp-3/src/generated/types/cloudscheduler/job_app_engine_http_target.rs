#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobAppEngineHttpTarget {
    /// App Engine Routing setting for the job.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "appEngineRouting")]
    pub r#app_engine_routing: Box<Option<super::super::types::cloudscheduler::JobAppEngineHttpTargetAppEngineRouting>>,
    /// HTTP request body.
    /// A request body is allowed only if the HTTP method is POST or PUT.
    /// It will result in invalid argument error to set a body on a job with an incompatible HttpMethod.
    /// A base64-encoded string.
    #[builder(into, default)]
    #[serde(rename = "body")]
    pub r#body: Box<Option<String>>,
    /// HTTP request headers.
    /// This map contains the header field names and values.
    /// Headers can be set when the job is created.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Which HTTP method to use for the request.
    #[builder(into, default)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Box<Option<String>>,
    /// The relative URI.
    /// The relative URL must begin with "/" and must be a valid HTTP relative URL.
    /// It can contain a path, query string arguments, and \# fragments.
    /// If the relative URL is empty, then the root path "/" will be used.
    /// No spaces are allowed, and the maximum length allowed is 2083 characters
    #[builder(into)]
    #[serde(rename = "relativeUri")]
    pub r#relative_uri: Box<String>,
}
