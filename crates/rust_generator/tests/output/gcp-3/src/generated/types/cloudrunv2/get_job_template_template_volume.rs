#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobTemplateTemplateVolume {
    /// For Cloud SQL volumes, contains the specific instances that should be mounted. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run.
    #[builder(into)]
    #[serde(rename = "cloudSqlInstances")]
    pub r#cloud_sql_instances: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeCloudSqlInstance>>,
    /// Ephemeral storage used as a shared volume.
    #[builder(into)]
    #[serde(rename = "emptyDirs")]
    pub r#empty_dirs: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeEmptyDir>>,
    /// Cloud Storage bucket mounted as a volume using GCSFuse.
    #[builder(into)]
    #[serde(rename = "gcs")]
    pub r#gcs: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeGc>>,
    /// The name of the Cloud Run v2 Job.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// NFS share mounted as a volume.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeNf>>,
    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateVolumeSecret>>,
}
