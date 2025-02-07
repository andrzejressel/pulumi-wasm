#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EngineSplitTrafficSplit {
    /// Mapping from version IDs within the service to fractional (0.000, 1] allocations of traffic for that version. Each version can be specified only once, but some versions in the service may not have any traffic allocation. Services that have traffic allocated cannot be deleted until either the service is deleted or their traffic allocation is removed. Allocations must sum to 1. Up to two decimal place precision is supported for IP-based splits and up to three decimal places is supported for cookie-based splits.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "allocations")]
    pub r#allocations: Box<std::collections::HashMap<String, String>>,
    /// Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed.
    /// Possible values are: `UNSPECIFIED`, `COOKIE`, `IP`, `RANDOM`.
    #[builder(into, default)]
    #[serde(rename = "shardBy")]
    pub r#shard_by: Box<Option<String>>,
}
