#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkConnectionMonitorTestConfigurationHttpConfiguration {
    /// The HTTP method for the HTTP request. Possible values are `Get` and `Post`. Defaults to `Get`.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// The path component of the URI. It only accepts the absolute path.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The port for the HTTP connection.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Should HTTPS be preferred over HTTP in cases where the choice is not explicit? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "preferHttps")]
    pub r#prefer_https: Box<Option<bool>>,
    /// A `request_header` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Box<Option<Vec<super::super::types::network::NetworkConnectionMonitorTestConfigurationHttpConfigurationRequestHeader>>>,
    /// The HTTP status codes to consider successful. For instance, `2xx`, `301-304` and `418`.
    #[builder(into, default)]
    #[serde(rename = "validStatusCodeRanges")]
    pub r#valid_status_code_ranges: Box<Option<Vec<String>>>,
}
