#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RoutingIntentRoutingPolicy {
    /// A list of destinations which this routing policy is applicable to. Possible values are `Internet` and `PrivateTraffic`.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Vec<String>>,
    /// The unique name for the routing policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource ID of the next hop on which this routing policy is applicable to.
    #[builder(into)]
    #[serde(rename = "nextHop")]
    pub r#next_hop: Box<String>,
}
