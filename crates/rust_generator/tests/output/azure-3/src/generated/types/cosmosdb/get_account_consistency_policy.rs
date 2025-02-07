#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountConsistencyPolicy {
    /// The Consistency Level used by this CosmosDB Account.
    #[builder(into)]
    #[serde(rename = "consistencyLevel")]
    pub r#consistency_level: Box<String>,
    /// The amount of staleness (in seconds) tolerated when the consistency level is Bounded Staleness.
    #[builder(into)]
    #[serde(rename = "maxIntervalInSeconds")]
    pub r#max_interval_in_seconds: Box<i32>,
    /// The number of stale requests tolerated when the consistency level is Bounded Staleness.
    #[builder(into)]
    #[serde(rename = "maxStalenessPrefix")]
    pub r#max_staleness_prefix: Box<i32>,
}
