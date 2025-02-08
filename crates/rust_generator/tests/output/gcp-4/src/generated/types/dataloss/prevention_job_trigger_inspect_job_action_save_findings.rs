#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobActionSaveFindings {
    /// Information on where to store output
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "outputConfig")]
    pub r#output_config: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig>,
}
