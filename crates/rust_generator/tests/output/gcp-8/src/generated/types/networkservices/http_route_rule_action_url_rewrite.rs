#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HttpRouteRuleActionUrlRewrite {
    /// Prior to forwarding the request to the selected destination, the requests host header is replaced by this value.
    #[builder(into, default)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Box<Option<String>>,
    /// Prior to forwarding the request to the selected destination, the matching portion of the requests path is replaced by this value.
    #[builder(into, default)]
    #[serde(rename = "pathPrefixRewrite")]
    pub r#path_prefix_rewrite: Box<Option<String>>,
}
