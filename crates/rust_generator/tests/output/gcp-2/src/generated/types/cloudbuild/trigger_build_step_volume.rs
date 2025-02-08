#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TriggerBuildStepVolume {
    /// Name of the volume to mount.
    /// Volume names must be unique per build step and must be valid names for Docker volumes.
    /// Each named volume must be used by at least two build steps.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Path at which to mount the volume.
    /// Paths must be absolute and cannot conflict with other volume paths on the same
    /// build step or with certain reserved volume paths.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
