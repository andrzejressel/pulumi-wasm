#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionLivenessCheck {
    /// Interval between health checks.
    #[builder(into, default)]
    #[serde(rename = "checkInterval")]
    pub r#check_interval: Box<Option<String>>,
    /// Number of consecutive failed checks required before considering the VM unhealthy. Default: 4.
    #[builder(into, default)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Box<Option<f64>>,
    /// Host header to send when performing a HTTP Readiness check. Example: "myapp.appspot.com"
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// The initial delay before starting to execute the checks. Default: "300s"
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "initialDelay")]
    pub r#initial_delay: Box<Option<String>>,
    /// The request path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Number of consecutive successful checks required before considering the VM healthy. Default: 2.
    #[builder(into, default)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Box<Option<f64>>,
    /// Time before the check is considered failed. Default: "4s"
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
