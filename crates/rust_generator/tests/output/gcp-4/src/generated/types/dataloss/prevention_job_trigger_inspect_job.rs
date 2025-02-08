#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJob {
    /// Configuration block for the actions to execute on the completion of a job. Can be specified multiple times, but only one for each type. Each action block supports fields documented below. This argument is processed in attribute-as-blocks mode.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobAction>>>,
    /// The core content of the template.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "inspectConfig")]
    pub r#inspect_config: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfig>>,
    /// The name of the template to run when this job is triggered.
    #[builder(into, default)]
    #[serde(rename = "inspectTemplateName")]
    pub r#inspect_template_name: Box<Option<String>>,
    /// Information on where to inspect
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageConfig")]
    pub r#storage_config: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfig>,
}
