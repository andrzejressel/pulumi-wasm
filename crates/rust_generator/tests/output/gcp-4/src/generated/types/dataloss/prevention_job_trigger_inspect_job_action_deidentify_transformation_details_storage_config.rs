#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobActionDeidentifyTransformationDetailsStorageConfig {
    /// The BigQuery table in which to store the output.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentifyTransformationDetailsStorageConfigTable>,
}
