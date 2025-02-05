#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayProbe {
    /// The Hostname used for this Probe. If the Application Gateway is configured for a single site, by default the Host name should be specified as `127.0.0.1`, unless otherwise configured in custom probe. Cannot be set if `pick_host_name_from_backend_http_settings` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Interval between two consecutive probes in seconds. Possible values range from 1 second to a maximum of 86,400 seconds.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// A `match` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<super::super::types::network::ApplicationGatewayProbeMatch>>,
    /// The minimum number of servers that are always marked as healthy. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "minimumServers")]
    pub r#minimum_servers: Box<Option<i32>>,
    /// The Name of the Probe.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Path used for this Probe.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Whether the host header should be picked from the backend HTTP settings. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "pickHostNameFromBackendHttpSettings")]
    pub r#pick_host_name_from_backend_http_settings: Box<Option<bool>>,
    /// Custom port which will be used for probing the backend servers. The valid value ranges from 1 to 65535. In case not set, port from HTTP settings will be used. This property is valid for Basic, Standard_v2 and WAF_v2 only.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The Protocol used for this Probe. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The Timeout used for this Probe, which indicates when a probe becomes unhealthy. Possible values range from 1 second to a maximum of 86,400 seconds.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<i32>,
    /// The Unhealthy Threshold for this Probe, which indicates the amount of retries which should be attempted before a node is deemed unhealthy. Possible values are from 1 to 20.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<i32>,
}
