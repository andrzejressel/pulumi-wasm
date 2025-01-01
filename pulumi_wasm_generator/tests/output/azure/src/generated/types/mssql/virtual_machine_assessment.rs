#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineAssessment {
    /// Should Assessment be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Should Assessment be run immediately? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "runImmediately")]
    pub r#run_immediately: Box<Option<bool>>,
    /// An `schedule` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<super::super::types::mssql::VirtualMachineAssessmentSchedule>>,
}
