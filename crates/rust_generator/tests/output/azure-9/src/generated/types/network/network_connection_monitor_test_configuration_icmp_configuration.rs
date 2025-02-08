#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkConnectionMonitorTestConfigurationIcmpConfiguration {
    /// Should path evaluation with trace route be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "traceRouteEnabled")]
    pub r#trace_route_enabled: Box<Option<bool>>,
}
