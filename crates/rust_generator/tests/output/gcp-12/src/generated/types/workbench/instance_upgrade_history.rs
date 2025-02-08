#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceUpgradeHistory {
    /// Optional. Action. Rolloback or Upgrade.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Optional. The container image before this instance upgrade.
    #[builder(into, default)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Box<Option<String>>,
    /// An RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
    /// The milliseconds portion (".SSS") is optional.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Optional. The framework of this workbench instance.
    #[builder(into, default)]
    #[serde(rename = "framework")]
    pub r#framework: Box<Option<String>>,
    /// Optional. The snapshot of the boot disk of this workbench instance before upgrade.
    #[builder(into, default)]
    #[serde(rename = "snapshot")]
    pub r#snapshot: Box<Option<String>>,
    /// (Output)
    /// Output only. The state of this instance upgrade history entry.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// Optional. Target VM Version, like m63.
    #[builder(into, default)]
    #[serde(rename = "targetVersion")]
    pub r#target_version: Box<Option<String>>,
    /// Optional. The version of the workbench instance before this upgrade.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
    /// Optional. The VM image before this instance upgrade.
    #[builder(into, default)]
    #[serde(rename = "vmImage")]
    pub r#vm_image: Box<Option<String>>,
}
