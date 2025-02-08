#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateVolume {
    /// For Cloud SQL volumes, contains the specific instances that should be mounted. Visit https://cloud.google.com/sql/docs/mysql/connect-run for more information on how to connect Cloud SQL and Cloud Run.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudSqlInstance")]
    pub r#cloud_sql_instance: Box<Option<super::super::types::cloudrunv2::ServiceTemplateVolumeCloudSqlInstance>>,
    /// Ephemeral storage used as a shared volume.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "emptyDir")]
    pub r#empty_dir: Box<Option<super::super::types::cloudrunv2::ServiceTemplateVolumeEmptyDir>>,
    /// Cloud Storage bucket mounted as a volume using GCSFuse. This feature is only supported in the gen2 execution environment.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcs")]
    pub r#gcs: Box<Option<super::super::types::cloudrunv2::ServiceTemplateVolumeGcs>>,
    /// Volume's name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Represents an NFS mount.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Option<super::super::types::cloudrunv2::ServiceTemplateVolumeNfs>>,
    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<super::super::types::cloudrunv2::ServiceTemplateVolumeSecret>>,
}
