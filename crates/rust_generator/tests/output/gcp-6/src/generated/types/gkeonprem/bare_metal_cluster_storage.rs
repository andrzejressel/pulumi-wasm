#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BareMetalClusterStorage {
    /// Specifies the config for local PersistentVolumes backed
    /// by mounted node disks. These disks need to be formatted and mounted by the
    /// user, which can be done before or after cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lvpNodeMountsConfig")]
    pub r#lvp_node_mounts_config: Box<super::super::types::gkeonprem::BareMetalClusterStorageLvpNodeMountsConfig>,
    /// Specifies the config for local PersistentVolumes backed by
    /// subdirectories in a shared filesystem. These subdirectores are
    /// automatically created during cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lvpShareConfig")]
    pub r#lvp_share_config: Box<super::super::types::gkeonprem::BareMetalClusterStorageLvpShareConfig>,
}
