#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerHealthCheck {
    /// The number of checks before the instance is declared healthy.
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Box<i32>,
    /// The interval between checks.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    /// The target of the check. Valid pattern is "${PROTOCOL}:${PORT}${PATH}", where PROTOCOL
    /// values are:
    /// * `HTTP`, `HTTPS` - PORT and PATH are required
    /// * `TCP`, `SSL` - PORT is required, PATH is not supported
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The length of time before the check times out.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<i32>,
    /// The number of checks before the instance is declared unhealthy.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<i32>,
}
