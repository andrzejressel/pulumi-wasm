#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobActionSaveFindings {
    /// Information on where to store output
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "outputConfig")]
    pub r#output_config: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig>,
}
