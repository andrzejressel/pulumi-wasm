#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceTemplateSchedulingMaxRunDuration {
    /// Span of time that's a fraction of a second at nanosecond
    /// resolution. Durations less than one second are represented
    /// with a 0 seconds field and a positive nanos field. Must
    /// be from 0 to 999,999,999 inclusive.
    #[builder(into)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<i32>,
    /// Span of time at a resolution of a second.
    /// Must be from 0 to 315,576,000,000 inclusive.
    #[builder(into)]
    #[serde(rename = "seconds")]
    pub r#seconds: Box<i32>,
}
