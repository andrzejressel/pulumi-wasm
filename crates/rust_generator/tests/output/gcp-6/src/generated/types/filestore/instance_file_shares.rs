#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFileShares {
    /// File share capacity in GiB. This must be at least 1024 GiB
    /// for the standard tier, or 2560 GiB for the premium tier.
    #[builder(into)]
    #[serde(rename = "capacityGb")]
    pub r#capacity_gb: Box<i32>,
    /// The name of the fileshare (16 characters or less)
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Nfs Export Options. There is a limit of 10 export options per file share.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nfsExportOptions")]
    pub r#nfs_export_options: Box<Option<Vec<super::super::types::filestore::InstanceFileSharesNfsExportOption>>>,
    /// The resource name of the backup, in the format
    /// projects/{projectId}/locations/{locationId}/backups/{backupId},
    /// that this file share has been restored from.
    #[builder(into, default)]
    #[serde(rename = "sourceBackup")]
    pub r#source_backup: Box<Option<String>>,
}
