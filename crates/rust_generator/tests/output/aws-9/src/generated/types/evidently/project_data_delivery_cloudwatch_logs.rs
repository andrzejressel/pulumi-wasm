#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProjectDataDeliveryCloudwatchLogs {
    /// The name of the log group where the project stores evaluation events.
    #[builder(into, default)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<Option<String>>,
}
