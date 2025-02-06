#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanExecutionSpecTrigger {
    /// The scan runs once via dataScans.run API.
    #[builder(into, default)]
    #[serde(rename = "onDemand")]
    pub r#on_demand: Box<Option<super::super::types::dataplex::DatascanExecutionSpecTriggerOnDemand>>,
    /// The scan is scheduled to run periodically.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<super::super::types::dataplex::DatascanExecutionSpecTriggerSchedule>>,
}
