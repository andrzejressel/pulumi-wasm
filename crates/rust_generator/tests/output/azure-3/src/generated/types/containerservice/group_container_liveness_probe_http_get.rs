#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GroupContainerLivenessProbeHttpGet {
    /// A map of HTTP headers used to access on the container. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Path to access on the HTTP server. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Number of the port to access on the container. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Scheme to use for connecting to the host. Possible values are `Http` and `Https`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "scheme")]
    pub r#scheme: Box<Option<String>>,
}
