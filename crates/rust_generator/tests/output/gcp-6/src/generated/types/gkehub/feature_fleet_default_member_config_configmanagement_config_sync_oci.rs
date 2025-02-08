#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci {
    /// The Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount
    #[builder(into, default)]
    #[serde(rename = "gcpServiceAccountEmail")]
    pub r#gcp_service_account_email: Box<Option<String>>,
    /// The absolute path of the directory that contains the local resources. Default: the root directory of the image
    #[builder(into, default)]
    #[serde(rename = "policyDir")]
    pub r#policy_dir: Box<Option<String>>,
    /// Type of secret configured for access to the Git repo
    #[builder(into)]
    #[serde(rename = "secretType")]
    pub r#secret_type: Box<String>,
    /// The OCI image repository URL for the package to sync from
    #[builder(into, default)]
    #[serde(rename = "syncRepo")]
    pub r#sync_repo: Box<Option<String>>,
    /// Period in seconds between consecutive syncs. Default: 15
    #[builder(into, default)]
    #[serde(rename = "syncWaitSecs")]
    pub r#sync_wait_secs: Box<Option<String>>,
    /// (Optional, Deprecated)
    /// Version of Config Sync installed
    /// 
    /// > **Warning:** The `configmanagement.config_sync.oci.version` field is deprecated and will be removed in a future major release. Please use `configmanagement.version` field to specify the version of Config Sync installed instead.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
