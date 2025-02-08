#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerRuleOverridePopPool {
    /// A list of pool IDs in failover priority to use for traffic reaching the given PoP.
    #[builder(into)]
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
    /// A 3-letter code for the Point-of-Presence. Allowed values can be found in the list of datacenters on the [status page](https://www.cloudflarestatus.com/). Multiple entries should not be specified with the same PoP.
    #[builder(into)]
    #[serde(rename = "pop")]
    pub r#pop: Box<String>,
}
