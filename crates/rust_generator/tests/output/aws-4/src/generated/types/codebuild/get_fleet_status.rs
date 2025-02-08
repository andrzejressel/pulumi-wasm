#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFleetStatus {
    /// Additional information about a compute fleet.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Box<String>,
    /// Message associated with the status of a compute fleet.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<String>,
    /// Status code of the compute fleet.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
