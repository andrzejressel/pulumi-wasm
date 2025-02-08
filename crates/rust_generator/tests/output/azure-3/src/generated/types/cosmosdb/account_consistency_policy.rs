#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountConsistencyPolicy {
    /// The Consistency Level to use for this CosmosDB Account - can be either `BoundedStaleness`, `Eventual`, `Session`, `Strong` or `ConsistentPrefix`.
    #[builder(into)]
    #[serde(rename = "consistencyLevel")]
    pub r#consistency_level: Box<String>,
    /// When used with the Bounded Staleness consistency level, this value represents the time amount of staleness (in seconds) tolerated. The accepted range for this value is `5` - `86400` (1 day). Defaults to `5`. Required when `consistency_level` is set to `BoundedStaleness`.
    #[builder(into, default)]
    #[serde(rename = "maxIntervalInSeconds")]
    pub r#max_interval_in_seconds: Box<Option<i32>>,
    /// When used with the Bounded Staleness consistency level, this value represents the number of stale requests tolerated. The accepted range for this value is `10` – `2147483647`. Defaults to `100`. Required when `consistency_level` is set to `BoundedStaleness`.
    /// 
    /// > **Note:** `max_interval_in_seconds` and `max_staleness_prefix` can only be set to values other than default when the `consistency_level` is set to `BoundedStaleness`.
    #[builder(into, default)]
    #[serde(rename = "maxStalenessPrefix")]
    pub r#max_staleness_prefix: Box<Option<i32>>,
}
