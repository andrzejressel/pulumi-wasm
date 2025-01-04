#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchExecution {
    /// The date and time that the launch ended.
    #[builder(into, default)]
    #[serde(rename = "endedTime")]
    pub r#ended_time: Box<Option<String>>,
    /// The date and time that the launch started.
    #[builder(into, default)]
    #[serde(rename = "startedTime")]
    pub r#started_time: Box<Option<String>>,
}
