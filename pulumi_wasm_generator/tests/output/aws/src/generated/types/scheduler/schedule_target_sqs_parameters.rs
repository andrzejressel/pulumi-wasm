#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScheduleTargetSqsParameters {
    /// FIFO message group ID to use as the target.
    #[builder(into, default)]
    #[serde(rename = "messageGroupId")]
    pub r#message_group_id: Box<Option<String>>,
}