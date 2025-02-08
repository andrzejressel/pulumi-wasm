#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListingRestrictedExportConfig {
    /// If true, enable restricted export.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// (Output)
    /// If true, restrict direct table access(read api/tabledata.list) on linked table.
    #[builder(into, default)]
    #[serde(rename = "restrictDirectTableAccess")]
    pub r#restrict_direct_table_access: Box<Option<bool>>,
    /// If true, restrict export of query result derived from restricted linked dataset table.
    #[builder(into, default)]
    #[serde(rename = "restrictQueryResult")]
    pub r#restrict_query_result: Box<Option<bool>>,
}
