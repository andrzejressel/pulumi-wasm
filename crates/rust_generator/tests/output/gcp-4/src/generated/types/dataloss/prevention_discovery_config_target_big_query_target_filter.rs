#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetFilter {
    /// Catch-all. This should always be the last filter in the list because anything above it will apply first.
    #[builder(into, default)]
    #[serde(rename = "otherTables")]
    pub r#other_tables: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterOtherTables>>,
    /// The table to scan. Discovery configurations including this can only include one DiscoveryTarget (the DiscoveryTarget with this TableReference).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tableReference")]
    pub r#table_reference: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterTableReference>>,
    /// A specific set of tables for this filter to apply to. A table collection must be specified in only one filter per config.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tables")]
    pub r#tables: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterTables>>,
}
