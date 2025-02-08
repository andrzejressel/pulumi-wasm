#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KubernetesClusterStorageProfile {
    /// Is the Blob CSI driver enabled? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "blobDriverEnabled")]
    pub r#blob_driver_enabled: Box<Option<bool>>,
    /// Is the Disk CSI driver enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "diskDriverEnabled")]
    pub r#disk_driver_enabled: Box<Option<bool>>,
    /// Is the File CSI driver enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "fileDriverEnabled")]
    pub r#file_driver_enabled: Box<Option<bool>>,
    /// Is the Snapshot Controller enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "snapshotControllerEnabled")]
    pub r#snapshot_controller_enabled: Box<Option<bool>>,
}
