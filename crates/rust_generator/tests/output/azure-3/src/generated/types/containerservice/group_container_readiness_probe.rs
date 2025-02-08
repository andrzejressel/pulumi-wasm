#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupContainerReadinessProbe {
    /// Commands to be run to validate container readiness. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "execs")]
    pub r#execs: Box<Option<Vec<String>>>,
    /// How many times to try the probe before restarting the container (liveness probe) or marking the container as unhealthy (readiness probe). Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Box<Option<i32>>,
    /// The definition of the http_get for this container as documented in the `http_get` block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "httpGets")]
    pub r#http_gets: Box<Option<Vec<super::super::types::containerservice::GroupContainerReadinessProbeHttpGet>>>,
    /// Number of seconds after the container has started before liveness or readiness probes are initiated. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "initialDelaySeconds")]
    pub r#initial_delay_seconds: Box<Option<i32>>,
    /// How often (in seconds) to perform the probe. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "periodSeconds")]
    pub r#period_seconds: Box<Option<i32>>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Box<Option<i32>>,
    /// Number of seconds after which the probe times out. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
}
