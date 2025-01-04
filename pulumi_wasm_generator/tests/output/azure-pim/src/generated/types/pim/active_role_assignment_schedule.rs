#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActiveRoleAssignmentSchedule {
    /// An `expiration` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<super::super::types::pim::ActiveRoleAssignmentScheduleExpiration>>,
    /// The start date/time of the role assignment. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "startDateTime")]
    pub r#start_date_time: Box<Option<String>>,
}
