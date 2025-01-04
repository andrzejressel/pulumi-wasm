#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetGenerationCadence {
    /// Governs when to update data profiles when the inspection rules defined by the `InspectTemplate` change. If not set, changing the template will not cause a data profile to update.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "inspectTemplateModifiedCadence")]
    pub r#inspect_template_modified_cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTargetGenerationCadenceInspectTemplateModifiedCadence>>,
    /// Data changes in Cloud Storage can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying buckets have changes. Defaults to never.
    /// Possible values are: `UPDATE_FREQUENCY_NEVER`, `UPDATE_FREQUENCY_DAILY`, `UPDATE_FREQUENCY_MONTHLY`.
    #[builder(into, default)]
    #[serde(rename = "refreshFrequency")]
    pub r#refresh_frequency: Box<Option<String>>,
    /// Governs when to update data profiles when a schema is modified
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "schemaModifiedCadence")]
    pub r#schema_modified_cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTargetGenerationCadenceSchemaModifiedCadence>>,
}
