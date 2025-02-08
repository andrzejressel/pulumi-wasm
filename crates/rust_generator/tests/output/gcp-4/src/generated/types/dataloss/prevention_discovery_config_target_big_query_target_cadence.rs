#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetCadence {
    /// Governs when to update data profiles when the inspection rules defined by the `InspectTemplate` change. If not set, changing the template will not cause a data profile to update.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "inspectTemplateModifiedCadence")]
    pub r#inspect_template_modified_cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceInspectTemplateModifiedCadence>>,
    /// Governs when to update data profiles when a schema is modified
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "schemaModifiedCadence")]
    pub r#schema_modified_cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceSchemaModifiedCadence>>,
    /// Governs when to update profile when a table is modified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tableModifiedCadence")]
    pub r#table_modified_cadence: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetCadenceTableModifiedCadence>>,
}
