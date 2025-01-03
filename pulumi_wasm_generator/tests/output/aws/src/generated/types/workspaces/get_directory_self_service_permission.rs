#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDirectorySelfServicePermission {
    /// Whether WorkSpaces directory users can change the compute type (bundle) for their workspace.
    #[builder(into)]
    #[serde(rename = "changeComputeType")]
    pub r#change_compute_type: Box<bool>,
    /// Whether WorkSpaces directory users can increase the volume size of the drives on their workspace.
    #[builder(into)]
    #[serde(rename = "increaseVolumeSize")]
    pub r#increase_volume_size: Box<bool>,
    /// Whether WorkSpaces directory users can rebuild the operating system of a workspace to its original state.
    #[builder(into)]
    #[serde(rename = "rebuildWorkspace")]
    pub r#rebuild_workspace: Box<bool>,
    /// Whether WorkSpaces directory users can restart their workspace.
    #[builder(into)]
    #[serde(rename = "restartWorkspace")]
    pub r#restart_workspace: Box<bool>,
    /// Whether WorkSpaces directory users can switch the running mode of their workspace.
    #[builder(into)]
    #[serde(rename = "switchRunningMode")]
    pub r#switch_running_mode: Box<bool>,
}
