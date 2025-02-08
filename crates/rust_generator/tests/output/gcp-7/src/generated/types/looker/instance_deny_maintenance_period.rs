#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceDenyMaintenancePeriod {
    /// Required. Start date of the deny maintenance period
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<super::super::types::looker::InstanceDenyMaintenancePeriodEndDate>,
    /// Required. Start date of the deny maintenance period
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<super::super::types::looker::InstanceDenyMaintenancePeriodStartDate>,
    /// Required. Start time of the window in UTC time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<super::super::types::looker::InstanceDenyMaintenancePeriodTime>,
}
