#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactsRotationRecurrenceShiftCoverageCoverageTime {
    /// (Required) The end time of the on-call shift. See Hand Off Time for more details.
    #[builder(into, default)]
    #[serde(rename = "end")]
    pub r#end: Box<Option<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeEnd>>,
    /// (Required) The start time of the on-call shift. See Hand Off Time for more details.
    #[builder(into, default)]
    #[serde(rename = "start")]
    pub r#start: Box<Option<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeStart>>,
}