#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetApplicationGatewayProbe {
    /// The Hostname used for this Probe.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The Interval between two consecutive probes in seconds.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// A `match` block as defined above.
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::network::GetApplicationGatewayProbeMatch>>,
    /// The minimum number of servers that are always marked as healthy.
    #[builder(into)]
    #[serde(rename = "minimumServers")]
    pub r#minimum_servers: Box<i32>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Whether the host header is picked from the backend HTTP settings.
    #[builder(into)]
    #[serde(rename = "pickHostNameFromBackendHttpSettings")]
    pub r#pick_host_name_from_backend_http_settings: Box<bool>,
    /// Custom port which is used for probing the backend servers.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The Protocol used for this Probe.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The Timeout used for this Probe, indicating when a probe becomes unhealthy.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<i32>,
    /// The Unhealthy Threshold for this Probe, which indicates the amount of retries which will be attempted before a node is deemed unhealthy.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<i32>,
}
