#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterStorageProfile {
    /// Is the Blob CSI driver enabled?
    #[builder(into)]
    #[serde(rename = "blobDriverEnabled")]
    pub r#blob_driver_enabled: Box<bool>,
    /// Is the Disk CSI driver enabled?
    #[builder(into)]
    #[serde(rename = "diskDriverEnabled")]
    pub r#disk_driver_enabled: Box<bool>,
    /// Is the File CSI driver enabled?
    #[builder(into)]
    #[serde(rename = "fileDriverEnabled")]
    pub r#file_driver_enabled: Box<bool>,
    /// Is the Snapshot Controller enabled?
    #[builder(into)]
    #[serde(rename = "snapshotControllerEnabled")]
    pub r#snapshot_controller_enabled: Box<bool>,
}
