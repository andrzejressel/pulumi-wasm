#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipConfigmanagementConfigSyncOci {
    /// The GCP Service Account Email used for auth when secret_type is gcpserviceaccount.
    #[builder(into, default)]
    #[serde(rename = "gcpServiceAccountEmail")]
    pub r#gcp_service_account_email: Box<Option<String>>,
    /// The absolute path of the directory that contains the local resources. Default: the root directory of the image.
    #[builder(into, default)]
    #[serde(rename = "policyDir")]
    pub r#policy_dir: Box<Option<String>>,
    /// Type of secret configured for access to the OCI Image. Must be one of gcenode, gcpserviceaccount or none.
    #[builder(into, default)]
    #[serde(rename = "secretType")]
    pub r#secret_type: Box<Option<String>>,
    /// The OCI image repository URL for the package to sync from. e.g. LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME.
    #[builder(into, default)]
    #[serde(rename = "syncRepo")]
    pub r#sync_repo: Box<Option<String>>,
    /// Period in seconds(int64 format) between consecutive syncs. Default: 15.
    #[builder(into, default)]
    #[serde(rename = "syncWaitSecs")]
    pub r#sync_wait_secs: Box<Option<String>>,
}
