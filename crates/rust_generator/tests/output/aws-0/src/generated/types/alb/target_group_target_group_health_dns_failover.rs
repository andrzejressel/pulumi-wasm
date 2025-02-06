#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetGroupTargetGroupHealthDnsFailover {
    /// The minimum number of targets that must be healthy. If the number of healthy targets is below this value, mark the zone as unhealthy in DNS, so that traffic is routed only to healthy zones. The possible values are `off` or an integer from `1` to the maximum number of targets. The default is `off`.
    #[builder(into, default)]
    #[serde(rename = "minimumHealthyTargetsCount")]
    pub r#minimum_healthy_targets_count: Box<Option<String>>,
    /// The minimum percentage of targets that must be healthy. If the percentage of healthy targets is below this value, mark the zone as unhealthy in DNS, so that traffic is routed only to healthy zones. The possible values are `off` or an integer from `1` to `100`. The default is `off`.
    #[builder(into, default)]
    #[serde(rename = "minimumHealthyTargetsPercentage")]
    pub r#minimum_healthy_targets_percentage: Box<Option<String>>,
}
