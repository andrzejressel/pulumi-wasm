#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AppTemplateContainerReadinessProbe {
    /// The number of consecutive failures required to consider this probe as failed. Possible values are between `1` and `30`. Defaults to `3`.
    #[builder(into, default)]
    #[serde(rename = "failureCountThreshold")]
    pub r#failure_count_threshold: Box<Option<i32>>,
    /// A `header` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::containerapp::AppTemplateContainerReadinessProbeHeader>>>,
    /// The probe hostname. Defaults to the pod IP address. Setting a value for `Host` in `headers` can be used to override this for `HTTP` and `HTTPS` type probes.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// The number of seconds elapsed after the container has started before the probe is initiated. Possible values are between `0` and `60`. Defaults to `0` seconds.
    #[builder(into, default)]
    #[serde(rename = "initialDelay")]
    pub r#initial_delay: Box<Option<i32>>,
    /// How often, in seconds, the probe should run. Possible values are between `1` and `240`. Defaults to `10`
    #[builder(into, default)]
    #[serde(rename = "intervalSeconds")]
    pub r#interval_seconds: Box<Option<i32>>,
    /// The URI to use for http type probes. Not valid for `TCP` type probes. Defaults to `/`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The port number on which to connect. Possible values are between `1` and `65535`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The number of consecutive successful responses required to consider this probe as successful. Possible values are between `1` and `10`. Defaults to `3`.
    #[builder(into, default)]
    #[serde(rename = "successCountThreshold")]
    pub r#success_count_threshold: Box<Option<i32>>,
    /// Time in seconds after which the probe times out. Possible values are in the range `1` - `240`. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
    /// Type of probe. Possible values are `TCP`, `HTTP`, and `HTTPS`.
    #[builder(into)]
    #[serde(rename = "transport")]
    pub r#transport: Box<String>,
}
