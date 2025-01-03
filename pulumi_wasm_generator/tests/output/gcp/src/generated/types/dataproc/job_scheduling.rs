#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobScheduling {
    /// Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.
    #[builder(into)]
    #[serde(rename = "maxFailuresPerHour")]
    pub r#max_failures_per_hour: Box<i32>,
    /// Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.
    #[builder(into)]
    #[serde(rename = "maxFailuresTotal")]
    pub r#max_failures_total: Box<i32>,
}
