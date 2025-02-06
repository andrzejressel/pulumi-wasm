#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkgroupConfigurationEngineVersion {
    /// The engine version on which the query runs. If `selected_engine_version` is set to `AUTO`, the effective engine version is chosen by Athena.
    #[builder(into, default)]
    #[serde(rename = "effectiveEngineVersion")]
    pub r#effective_engine_version: Box<Option<String>>,
    /// Requested engine version. Defaults to `AUTO`.
    #[builder(into, default)]
    #[serde(rename = "selectedEngineVersion")]
    pub r#selected_engine_version: Box<Option<String>>,
}
