#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduleTargetKinesisParameters {
    /// Specifies the shard to which EventBridge Scheduler sends the event. Up to 256 characters.
    #[builder(into)]
    #[serde(rename = "partitionKey")]
    pub r#partition_key: Box<String>,
}
