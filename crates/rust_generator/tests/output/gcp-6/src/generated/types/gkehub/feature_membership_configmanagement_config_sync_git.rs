#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipConfigmanagementConfigSyncGit {
    /// The GCP Service Account Email used for auth when secretType is gcpServiceAccount.
    #[builder(into, default)]
    #[serde(rename = "gcpServiceAccountEmail")]
    pub r#gcp_service_account_email: Box<Option<String>>,
    /// URL for the HTTPS proxy to be used when communicating with the Git repo.
    #[builder(into, default)]
    #[serde(rename = "httpsProxy")]
    pub r#https_proxy: Box<Option<String>>,
    /// The path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository.
    #[builder(into, default)]
    #[serde(rename = "policyDir")]
    pub r#policy_dir: Box<Option<String>>,
    /// Type of secret configured for access to the Git repo.
    #[builder(into, default)]
    #[serde(rename = "secretType")]
    pub r#secret_type: Box<Option<String>>,
    /// The branch of the repository to sync from. Default: master.
    #[builder(into, default)]
    #[serde(rename = "syncBranch")]
    pub r#sync_branch: Box<Option<String>>,
    /// The URL of the Git repository to use as the source of truth.
    #[builder(into, default)]
    #[serde(rename = "syncRepo")]
    pub r#sync_repo: Box<Option<String>>,
    /// Git revision (tag or hash) to check out. Default HEAD.
    #[builder(into, default)]
    #[serde(rename = "syncRev")]
    pub r#sync_rev: Box<Option<String>>,
    /// Period in seconds between consecutive syncs. Default: 15.
    #[builder(into, default)]
    #[serde(rename = "syncWaitSecs")]
    pub r#sync_wait_secs: Box<Option<String>>,
}
