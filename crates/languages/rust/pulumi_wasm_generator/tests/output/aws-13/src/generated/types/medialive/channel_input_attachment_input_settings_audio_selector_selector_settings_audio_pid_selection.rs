#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioPidSelection {
    /// Selects a specific PID from within a source.
    #[builder(into)]
    #[serde(rename = "pid")]
    pub r#pid: Box<i32>,
}
