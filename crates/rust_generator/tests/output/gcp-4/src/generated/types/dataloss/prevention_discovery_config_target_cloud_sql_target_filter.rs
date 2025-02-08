#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetFilter {
    /// A specific set of buckets for this filter to apply to.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "collection")]
    pub r#collection: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTargetFilterCollection>>,
    /// The database resource to scan. Targets including this can only include one target (the target with this database resource reference).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "databaseResourceReference")]
    pub r#database_resource_reference: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTargetFilterDatabaseResourceReference>>,
    /// Match discovery resources not covered by any other filter.
    #[builder(into, default)]
    #[serde(rename = "others")]
    pub r#others: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudSqlTargetFilterOthers>>,
}
