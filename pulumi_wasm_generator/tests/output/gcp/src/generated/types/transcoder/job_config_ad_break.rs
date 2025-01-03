#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobConfigAdBreak {
    /// Start time in seconds for the ad break, relative to the output file timeline
    #[builder(into, default)]
    #[serde(rename = "startTimeOffset")]
    pub r#start_time_offset: Box<Option<String>>,
}
