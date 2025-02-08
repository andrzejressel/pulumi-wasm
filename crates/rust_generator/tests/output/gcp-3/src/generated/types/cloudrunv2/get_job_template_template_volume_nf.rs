#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobTemplateTemplateVolumeNf {
    /// Path that is exported by the NFS server.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// If true, mount this volume as read-only in all mounts.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<bool>,
    /// Hostname or IP address of the NFS server.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
}
