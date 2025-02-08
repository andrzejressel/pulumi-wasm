#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFrontdoorOriginGroupHealthProbe {
    /// Specifies the number of seconds between health probes.
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<i32>,
    /// Specifies the path relative to the origin that is used to determine the health of the origin.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Specifies the protocol to use for health probe.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Specifies the type of health probe request that is made.
    #[builder(into)]
    #[serde(rename = "requestType")]
    pub r#request_type: Box<String>,
}
