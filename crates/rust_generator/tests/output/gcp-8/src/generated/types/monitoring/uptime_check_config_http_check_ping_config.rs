#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UptimeCheckConfigHttpCheckPingConfig {
    /// Number of ICMP pings. A maximum of 3 ICMP pings is currently supported.
    #[builder(into)]
    #[serde(rename = "pingsCount")]
    pub r#pings_count: Box<i32>,
}
