#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetWorkspaceWorkspaceProperty {
    /// Compute type. For more information, see [Amazon WorkSpaces Bundles](http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles). Valid values are `VALUE`, `STANDARD`, `PERFORMANCE`, `POWER`, `GRAPHICS`, `POWERPRO` and `GRAPHICSPRO`.
    #[builder(into)]
    #[serde(rename = "computeTypeName")]
    pub r#compute_type_name: Box<String>,
    /// Size of the root volume.
    #[builder(into)]
    #[serde(rename = "rootVolumeSizeGib")]
    pub r#root_volume_size_gib: Box<i32>,
    /// Running mode. For more information, see [Manage the WorkSpace Running Mode](https://docs.aws.amazon.com/workspaces/latest/adminguide/running-mode.html). Valid values are `AUTO_STOP` and `ALWAYS_ON`.
    #[builder(into)]
    #[serde(rename = "runningMode")]
    pub r#running_mode: Box<String>,
    /// Time after a user logs off when WorkSpaces are automatically stopped. Configured in 60-minute intervals.
    #[builder(into)]
    #[serde(rename = "runningModeAutoStopTimeoutInMinutes")]
    pub r#running_mode_auto_stop_timeout_in_minutes: Box<i32>,
    /// Size of the user storage.
    #[builder(into)]
    #[serde(rename = "userVolumeSizeGib")]
    pub r#user_volume_size_gib: Box<i32>,
}
