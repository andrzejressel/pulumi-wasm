#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListenerDefaultActionRedirect {
    /// Hostname. This component is not percent-encoded. The hostname can contain `#{host}`. Defaults to `#{host}`.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// Absolute path, starting with the leading "/". This component is not percent-encoded. The path can contain #{host}, #{path}, and #{port}. Defaults to `/#{path}`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Port. Specify a value from `1` to `65535` or `#{port}`. Defaults to `#{port}`.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<String>>,
    /// Protocol. Valid values are `HTTP`, `HTTPS`, or `#{protocol}`. Defaults to `#{protocol}`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Query parameters, URL-encoded when necessary, but not percent-encoded. Do not include the leading "?". Defaults to `#{query}`.
    #[builder(into, default)]
    #[serde(rename = "query")]
    pub r#query: Box<Option<String>>,
    /// HTTP redirect code. The redirect is either permanent (`HTTP_301`) or temporary (`HTTP_302`).
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
