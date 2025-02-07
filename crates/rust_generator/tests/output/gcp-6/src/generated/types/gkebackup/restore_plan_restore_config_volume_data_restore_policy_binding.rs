#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RestorePlanRestoreConfigVolumeDataRestorePolicyBinding {
    /// Specifies the mechanism to be used to restore this volume data.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy
    /// for more information on each policy option.
    /// Possible values are: `RESTORE_VOLUME_DATA_FROM_BACKUP`, `REUSE_VOLUME_HANDLE_FROM_BACKUP`, `NO_VOLUME_DATA_RESTORATION`.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Box<String>,
    /// The volume type, as determined by the PVC's
    /// bound PV, to apply the policy to.
    /// Possible values are: `GCE_PERSISTENT_DISK`.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<String>,
}
