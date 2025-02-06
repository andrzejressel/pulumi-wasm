#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigError {
    /// A list of messages that carry the error details.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigErrorDetails>>,
    /// The times the error occurred. List includes the oldest timestamp and the last 9 timestamps.
    #[builder(into, default)]
    #[serde(rename = "timestamp")]
    pub r#timestamp: Box<Option<String>>,
}
