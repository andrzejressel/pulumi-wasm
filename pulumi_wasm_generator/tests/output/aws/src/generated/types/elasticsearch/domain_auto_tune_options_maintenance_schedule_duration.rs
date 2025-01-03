#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainAutoTuneOptionsMaintenanceScheduleDuration {
    /// The unit of time specifying the duration of an Auto-Tune maintenance window. Valid values: `HOURS`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// An integer specifying the value of the duration of an Auto-Tune maintenance window.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
