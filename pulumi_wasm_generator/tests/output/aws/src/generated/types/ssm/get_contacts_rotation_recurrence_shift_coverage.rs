#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetContactsRotationRecurrenceShiftCoverage {
    #[builder(into)]
    #[serde(rename = "coverageTimes")]
    pub r#coverage_times: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceShiftCoverageCoverageTime>>,
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: Box<String>,
}