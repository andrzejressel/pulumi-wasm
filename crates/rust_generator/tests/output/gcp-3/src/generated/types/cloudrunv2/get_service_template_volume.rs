#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateVolume {
    /// For Cloud SQL volumes, contains the specific instances that should be mounted. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run.
    #[builder(into)]
    #[serde(rename = "cloudSqlInstances")]
    pub r#cloud_sql_instances: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVolumeCloudSqlInstance>>,
    /// Ephemeral storage used as a shared volume.
    #[builder(into)]
    #[serde(rename = "emptyDirs")]
    pub r#empty_dirs: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVolumeEmptyDir>>,
    /// Cloud Storage bucket mounted as a volume using GCSFuse. This feature is only supported in the gen2 execution environment.
    #[builder(into)]
    #[serde(rename = "gcs")]
    pub r#gcs: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVolumeGc>>,
    /// The name of the Cloud Run v2 Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Represents an NFS mount.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVolumeNf>>,
    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateVolumeSecret>>,
}
