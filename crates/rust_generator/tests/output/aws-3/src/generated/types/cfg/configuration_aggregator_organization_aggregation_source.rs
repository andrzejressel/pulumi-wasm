#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationAggregatorOrganizationAggregationSource {
    /// If true, aggregate existing AWS Config regions and future regions.
    #[builder(into, default)]
    #[serde(rename = "allRegions")]
    pub r#all_regions: Box<Option<bool>>,
    /// List of source regions being aggregated.
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<String>>>,
    /// ARN of the IAM role used to retrieve AWS Organization details associated with the aggregator account.
    /// 
    /// Either `regions` or `all_regions` (as true) must be specified.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
