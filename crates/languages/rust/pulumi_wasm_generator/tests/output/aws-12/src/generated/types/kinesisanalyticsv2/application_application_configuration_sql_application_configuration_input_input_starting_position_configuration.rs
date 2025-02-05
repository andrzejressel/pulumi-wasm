#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputStartingPositionConfiguration {
    /// The starting position on the stream. Valid values: `LAST_STOPPED_POINT`, `NOW`, `TRIM_HORIZON`.
    #[builder(into, default)]
    #[serde(rename = "inputStartingPosition")]
    pub r#input_starting_position: Box<Option<String>>,
}
