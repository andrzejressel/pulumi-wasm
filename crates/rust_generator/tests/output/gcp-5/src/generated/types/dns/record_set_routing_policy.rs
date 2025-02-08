#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RecordSetRoutingPolicy {
    /// Specifies whether to enable fencing for geo queries.
    #[builder(into, default)]
    #[serde(rename = "enableGeoFencing")]
    pub r#enable_geo_fencing: Box<Option<bool>>,
    /// The configuration for Geolocation based routing policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "geos")]
    pub r#geos: Box<Option<Vec<super::super::types::dns::RecordSetRoutingPolicyGeo>>>,
    /// The configuration for a failover policy with global to regional failover. Queries are responded to with the global primary targets, but if none of the primary targets are healthy, then we fallback to a regional failover policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "primaryBackup")]
    pub r#primary_backup: Box<Option<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackup>>,
    /// The configuration for Weighted Round Robin based routing policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "wrrs")]
    pub r#wrrs: Box<Option<Vec<super::super::types::dns::RecordSetRoutingPolicyWrr>>>,
}
