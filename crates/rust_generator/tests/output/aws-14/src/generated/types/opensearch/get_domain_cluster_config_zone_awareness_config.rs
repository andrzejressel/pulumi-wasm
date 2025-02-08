#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDomainClusterConfigZoneAwarenessConfig {
    /// Number of availability zones used.
    #[builder(into)]
    #[serde(rename = "availabilityZoneCount")]
    pub r#availability_zone_count: Box<i32>,
}
