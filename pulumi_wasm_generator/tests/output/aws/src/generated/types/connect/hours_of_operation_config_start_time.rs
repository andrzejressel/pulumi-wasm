#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HoursOfOperationConfigStartTime {
    /// Specifies the hour of opening.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Box<i32>,
    /// Specifies the minute of opening.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<i32>,
}