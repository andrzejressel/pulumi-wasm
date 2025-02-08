#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetGenerationCadenceSchemaModifiedCadence {
    /// Frequency to regenerate data profiles when the schema is modified. Defaults to monthly.
    /// Possible values are: `UPDATE_FREQUENCY_NEVER`, `UPDATE_FREQUENCY_DAILY`, `UPDATE_FREQUENCY_MONTHLY`.
    #[builder(into, default)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<Option<String>>,
    /// The types of schema modifications to consider. Defaults to NEW_COLUMNS.
    /// Each value may be one of: `NEW_COLUMNS`, `REMOVED_COLUMNS`.
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<Vec<String>>>,
}
