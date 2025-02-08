#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig {
    /// Schema used for writing the findings for Inspect jobs. This field is only used for
    /// Inspect and must be unspecified for Risk jobs. Columns are derived from the Finding
    /// object. If appending to an existing table, any columns from the predefined schema
    /// that are missing will be added. No columns in the existing table will be deleted.
    /// If unspecified, then all available columns will be used for a new table or an (existing)
    /// table with no schema, and no changes will be made to an existing table that has a schema.
    /// Only for use with external storage.
    /// Possible values are: `BASIC_COLUMNS`, `GCS_COLUMNS`, `DATASTORE_COLUMNS`, `BIG_QUERY_COLUMNS`, `ALL_COLUMNS`.
    #[builder(into, default)]
    #[serde(rename = "outputSchema")]
    pub r#output_schema: Box<Option<String>>,
    /// Information on the location of the target BigQuery Table.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable>,
}
