#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactsRotationRecurrenceShiftCoverage {
    /// (Required) Information about when an on-call shift begins and ends. See Coverage Times for more details.
    #[builder(into, default)]
    #[serde(rename = "coverageTimes")]
    pub r#coverage_times: Box<Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTime>>>,
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: Box<String>,
}