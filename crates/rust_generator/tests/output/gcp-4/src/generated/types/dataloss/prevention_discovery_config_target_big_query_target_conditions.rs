#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetConditions {
    /// File store must have been created after this date. Used to avoid backfilling. A timestamp in RFC3339 UTC "Zulu" format with nanosecond resolution and upto nine fractional digits.
    #[builder(into, default)]
    #[serde(rename = "createdAfter")]
    pub r#created_after: Box<Option<String>>,
    /// At least one of the conditions must be true for a table to be scanned.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "orConditions")]
    pub r#or_conditions: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetConditionsOrConditions>>,
    /// Restrict discovery to categories of table types. Currently view, materialized view, snapshot and non-biglake external tables are supported.
    /// Possible values are: `BIG_QUERY_COLLECTION_ALL_TYPES`, `BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES`.
    #[builder(into, default)]
    #[serde(rename = "typeCollection")]
    pub r#type_collection: Box<Option<String>>,
    /// Data profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES].
    /// Each value may be one of: `DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES`, `DATABASE_RESOURCE_TYPE_TABLE`.
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetConditionsTypes>>,
}
