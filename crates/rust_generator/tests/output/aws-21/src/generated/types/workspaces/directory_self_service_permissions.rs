#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DirectorySelfServicePermissions {
    /// Whether WorkSpaces directory users can change the compute type (bundle) for their workspace. Default `false`.
    #[builder(into, default)]
    #[serde(rename = "changeComputeType")]
    pub r#change_compute_type: Box<Option<bool>>,
    /// Whether WorkSpaces directory users can increase the volume size of the drives on their workspace. Default `false`.
    #[builder(into, default)]
    #[serde(rename = "increaseVolumeSize")]
    pub r#increase_volume_size: Box<Option<bool>>,
    /// Whether WorkSpaces directory users can rebuild the operating system of a workspace to its original state. Default `false`.
    #[builder(into, default)]
    #[serde(rename = "rebuildWorkspace")]
    pub r#rebuild_workspace: Box<Option<bool>>,
    /// Whether WorkSpaces directory users can restart their workspace. Default `true`.
    #[builder(into, default)]
    #[serde(rename = "restartWorkspace")]
    pub r#restart_workspace: Box<Option<bool>>,
    /// Whether WorkSpaces directory users can switch the running mode of their workspace. Default `false`.
    #[builder(into, default)]
    #[serde(rename = "switchRunningMode")]
    pub r#switch_running_mode: Box<Option<bool>>,
}
