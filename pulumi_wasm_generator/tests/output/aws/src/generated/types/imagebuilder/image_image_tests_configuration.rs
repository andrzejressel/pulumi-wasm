#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageImageTestsConfiguration {
    /// Whether image tests are enabled. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "imageTestsEnabled")]
    pub r#image_tests_enabled: Box<Option<bool>>,
    /// Number of minutes before image tests time out. Valid values are between `60` and `1440`. Defaults to `720`.
    #[builder(into, default)]
    #[serde(rename = "timeoutMinutes")]
    pub r#timeout_minutes: Box<Option<i32>>,
}
