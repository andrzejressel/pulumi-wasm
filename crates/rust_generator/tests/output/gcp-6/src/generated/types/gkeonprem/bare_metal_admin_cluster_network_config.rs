#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalAdminClusterNetworkConfig {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "islandModeCidr")]
    pub r#island_mode_cidr: Box<Option<super::super::types::gkeonprem::BareMetalAdminClusterNetworkConfigIslandModeCidr>>,
}
