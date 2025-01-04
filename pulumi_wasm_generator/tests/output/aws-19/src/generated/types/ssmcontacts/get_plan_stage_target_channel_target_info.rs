#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPlanStageTargetChannelTargetInfo {
    #[builder(into)]
    #[serde(rename = "contactChannelId")]
    pub r#contact_channel_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "retryIntervalInMinutes")]
    pub r#retry_interval_in_minutes: Box<i32>,
}
