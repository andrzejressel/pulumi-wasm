#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetHoursOfOperationConfig {
    /// Day that the hours of operation applies to.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// End time block specifies the time that your contact center closes. The `end_time` is documented below.
    #[builder(into)]
    #[serde(rename = "endTimes")]
    pub r#end_times: Box<Vec<super::super::types::connect::GetHoursOfOperationConfigEndTime>>,
    /// Start time block specifies the time that your contact center opens. The `start_time` is documented below.
    #[builder(into)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Box<Vec<super::super::types::connect::GetHoursOfOperationConfigStartTime>>,
}
