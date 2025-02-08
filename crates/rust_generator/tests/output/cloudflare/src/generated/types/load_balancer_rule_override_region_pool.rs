#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerRuleOverrideRegionPool {
    /// A list of pool IDs in failover priority to use in the given region.
    #[builder(into)]
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A region code which must be in the list defined [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api/#list-of-load-balancer-regions). Multiple entries should not be specified with the same region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
