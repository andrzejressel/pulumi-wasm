#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VpnSiteO365Policy {
    /// A `traffic_category` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "trafficCategory")]
    pub r#traffic_category: Box<Option<super::super::types::network::VpnSiteO365PolicyTrafficCategory>>,
}
