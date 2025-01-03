#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectDataDeliveryCloudwatchLogs {
    /// The name of the log group where the project stores evaluation events.
    #[builder(into, default)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<Option<String>>,
}
