#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTrafficManagerProfileMonitorConfig {
    /// One or more `custom_header` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customHeaders")]
    pub r#custom_headers: Box<Vec<super::super::types::network::GetTrafficManagerProfileMonitorConfigCustomHeader>>,
    /// A list of status code ranges.
    #[builder(into)]
    #[serde(rename = "expectedStatusCodeRanges")]
    pub r#expected_status_code_ranges: Box<Vec<String>>,
    /// The interval used to check the endpoint health from a Traffic Manager probing agent.
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<i32>,
    /// The path used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The port number used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The protocol used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The amount of time the Traffic Manager probing agent should wait before considering that check a failure when a health check probe is sent to the endpoint.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<i32>,
    /// The number of failures a Traffic Manager probing agent tolerates before marking that endpoint as unhealthy.
    #[builder(into)]
    #[serde(rename = "toleratedNumberOfFailures")]
    pub r#tolerated_number_of_failures: Box<i32>,
}
