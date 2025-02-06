#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsIdentifyingField {
    /// Name describing the field.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
