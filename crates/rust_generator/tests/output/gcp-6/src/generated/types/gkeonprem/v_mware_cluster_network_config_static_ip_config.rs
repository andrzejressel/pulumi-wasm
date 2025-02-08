#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterNetworkConfigStaticIpConfig {
    /// Represents the configuration values for static IP allocation to nodes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ipBlocks")]
    pub r#ip_blocks: Box<Vec<super::super::types::gkeonprem::VMwareClusterNetworkConfigStaticIpConfigIpBlock>>,
}
