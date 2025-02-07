#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobTemplateTemplateVolumeGc {
    /// Name of the cloud storage bucket to back the volume. The resource service account must have permission to access the bucket.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// A list of flags to pass to the gcsfuse command for configuring this volume.
    /// Flags should be passed without leading dashes.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Vec<String>>,
    /// If true, mount this volume as read-only in all mounts. If false, mount this volume as read-write.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<bool>,
}
