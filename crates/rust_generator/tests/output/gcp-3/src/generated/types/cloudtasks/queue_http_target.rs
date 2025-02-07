#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueHttpTarget {
    /// HTTP target headers.
    /// This map contains the header field names and values.
    /// Headers will be set when running the CreateTask and/or BufferTask.
    /// These headers represent a subset of the headers that will be configured for the task's HTTP request.
    /// Some HTTP request headers will be ignored or replaced.
    /// Headers which can have multiple values (according to RFC2616) can be specified using comma-separated values.
    /// The size of the headers must be less than 80KB. Queue-level headers to override headers of all the tasks in the queue.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerOverrides")]
    pub r#header_overrides: Box<Option<Vec<super::super::types::cloudtasks::QueueHttpTargetHeaderOverride>>>,
    /// The HTTP method to use for the request.
    /// When specified, it overrides HttpRequest for the task.
    /// Note that if the value is set to GET the body of the task will be ignored at execution time.
    /// Possible values are: `HTTP_METHOD_UNSPECIFIED`, `POST`, `GET`, `HEAD`, `PUT`, `DELETE`, `PATCH`, `OPTIONS`.
    #[builder(into, default)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Box<Option<String>>,
    /// If specified, an OAuth token is generated and attached as the Authorization header in the HTTP request.
    /// This type of authorization should generally be used only when calling Google APIs hosted on *.googleapis.com.
    /// Note that both the service account email and the scope MUST be specified when using the queue-level authorization override.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauthToken")]
    pub r#oauth_token: Box<Option<super::super::types::cloudtasks::QueueHttpTargetOauthToken>>,
    /// If specified, an OIDC token is generated and attached as an Authorization header in the HTTP request.
    /// This type of authorization can be used for many scenarios, including calling Cloud Run, or endpoints where you intend to validate the token yourself.
    /// Note that both the service account email and the audience MUST be specified when using the queue-level authorization override.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Box<Option<super::super::types::cloudtasks::QueueHttpTargetOidcToken>>,
    /// URI override.
    /// When specified, overrides the execution URI for all the tasks in the queue.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "uriOverride")]
    pub r#uri_override: Box<Option<super::super::types::cloudtasks::QueueHttpTargetUriOverride>>,
}
