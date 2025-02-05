#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobNotificationProperty {
    /// After a job run starts, the number of minutes to wait before sending a job run delay notification.
    #[builder(into, default)]
    #[serde(rename = "notifyDelayAfter")]
    pub r#notify_delay_after: Box<Option<i32>>,
}
