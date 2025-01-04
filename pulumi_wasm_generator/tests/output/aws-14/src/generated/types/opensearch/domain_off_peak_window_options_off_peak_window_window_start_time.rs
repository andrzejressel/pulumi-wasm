#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainOffPeakWindowOptionsOffPeakWindowWindowStartTime {
    /// Starting hour of the 10-hour window for updates
    #[builder(into, default)]
    #[serde(rename = "hours")]
    pub r#hours: Box<Option<i32>>,
    /// Starting minute of the 10-hour window for updates
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<i32>>,
}
