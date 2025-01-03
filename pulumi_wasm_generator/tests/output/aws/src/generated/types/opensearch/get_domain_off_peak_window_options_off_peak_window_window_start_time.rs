#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainOffPeakWindowOptionsOffPeakWindowWindowStartTime {
    /// Starting hour of the 10-hour window for updates
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Box<i32>,
    /// Starting minute of the 10-hour window for updates
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<i32>,
}
