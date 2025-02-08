#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetContactsRotationRecurrenceShiftCoverage {
    #[builder(into)]
    #[serde(rename = "coverageTimes")]
    pub r#coverage_times: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceShiftCoverageCoverageTime>>,
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: Box<String>,
}
