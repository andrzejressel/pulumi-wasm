#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseInstanceSettingsDenyMaintenancePeriod {
    /// "deny maintenance period" end date. If the year of the end date is empty, the year of the start date also must be empty. In this case, it means the no maintenance interval recurs every year. The date is in format yyyy-m-dd (the month is without leading zeros)i.e., 2020-1-01, or 2020-11-01, or mm-dd, i.e., 11-01
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<String>,
    /// "deny maintenance period" start date. If the year of the start date is empty, the year of the end date also must be empty. In this case, it means the deny maintenance period recurs every year. The date is in format yyyy-m-dd (the month is without leading zeros)i.e., 2020-1-01, or 2020-11-01, or mm-dd, i.e., 11-01
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<String>,
    /// Time in UTC when the "deny maintenance period" starts on startDate and ends on endDate. The time is in format: HH:mm:SS, i.e., 00:00:00
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
}
