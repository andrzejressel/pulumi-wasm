#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HoursOfOperationConfig {
    /// Specifies the day that the hours of operation applies to.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// A end time block specifies the time that your contact center closes. The `end_time` is documented below.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<super::super::types::connect::HoursOfOperationConfigEndTime>,
    /// A start time block specifies the time that your contact center opens. The `start_time` is documented below.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<super::super::types::connect::HoursOfOperationConfigStartTime>,
}
