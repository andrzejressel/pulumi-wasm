#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RunBookJobSchedule {
    /// The UUID of automation runbook job schedule ID.
    #[builder(into, default)]
    #[serde(rename = "jobScheduleId")]
    pub r#job_schedule_id: Box<Option<String>>,
    /// A map of key/value pairs corresponding to the arguments that can be passed to the Runbook.
    /// 
    /// > **NOTE:** The parameter keys/names must strictly be in lowercase, even if this is not the case in the runbook. This is due to a limitation in Azure Automation where the parameter names are normalized. The values specified don't have this limitation.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Name of a Hybrid Worker Group the Runbook will be executed on.
    #[builder(into, default)]
    #[serde(rename = "runOn")]
    pub r#run_on: Box<Option<String>>,
    /// The name of the Schedule.
    #[builder(into)]
    #[serde(rename = "scheduleName")]
    pub r#schedule_name: Box<String>,
}
