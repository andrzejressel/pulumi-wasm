#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationAggregatorAccountAggregationSource {
    /// List of 12-digit account IDs of the account(s) being aggregated.
    #[builder(into)]
    #[serde(rename = "accountIds")]
    pub r#account_ids: Box<Vec<String>>,
    /// If true, aggregate existing AWS Config regions and future regions.
    #[builder(into, default)]
    #[serde(rename = "allRegions")]
    pub r#all_regions: Box<Option<bool>>,
    /// List of source regions being aggregated.
    /// 
    /// Either `regions` or `all_regions` (as true) must be specified.
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<String>>>,
}
