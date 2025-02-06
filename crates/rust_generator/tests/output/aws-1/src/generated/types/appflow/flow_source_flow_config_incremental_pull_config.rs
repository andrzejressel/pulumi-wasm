#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowSourceFlowConfigIncrementalPullConfig {
    /// Field that specifies the date time or timestamp field as the criteria to use when importing incremental records from the source.
    #[builder(into, default)]
    #[serde(rename = "datetimeTypeFieldName")]
    pub r#datetime_type_field_name: Box<Option<String>>,
}
