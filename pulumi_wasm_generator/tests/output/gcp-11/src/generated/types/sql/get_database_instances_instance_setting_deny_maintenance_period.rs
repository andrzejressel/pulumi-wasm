#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstancesInstanceSettingDenyMaintenancePeriod {
    /// End date before which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<String>,
    /// Start date after which maintenance will not take place. The date is in format yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<String>,
    /// Time in UTC when the "deny maintenance period" starts on start_date and ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
}
