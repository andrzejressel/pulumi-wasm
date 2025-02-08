#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceProtocols {
    /// Should HTTP/2 be supported by the API Management Service? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enableHttp2")]
    pub r#enable_http_2: Box<Option<bool>>,
}
