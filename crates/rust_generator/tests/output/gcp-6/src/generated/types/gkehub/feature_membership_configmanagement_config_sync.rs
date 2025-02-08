#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipConfigmanagementConfigSync {
    /// Enables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// (Optional) Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "git")]
    pub r#git: Box<Option<super::super::types::gkehub::FeatureMembershipConfigmanagementConfigSyncGit>>,
    /// Deprecated: If Workload Identity Federation for GKE is enabled, Google Cloud Service Account is no longer needed for exporting Config Sync metrics: https://cloud.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/monitor-config-sync-cloud-monitoring#custom-monitoring.
    #[builder(into, default)]
    #[serde(rename = "metricsGcpServiceAccountEmail")]
    pub r#metrics_gcp_service_account_email: Box<Option<String>>,
    /// (Optional) Supported from Config Sync versions 1.12.0 onwards. Structure is documented below.
    /// 
    /// Use either `git` or `oci` config option.
    #[builder(into, default)]
    #[serde(rename = "oci")]
    pub r#oci: Box<Option<super::super::types::gkehub::FeatureMembershipConfigmanagementConfigSyncOci>>,
    /// Supported from Config Sync versions 1.10.0 onwards. Set to `true` to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts.
    #[builder(into, default)]
    #[serde(rename = "preventDrift")]
    pub r#prevent_drift: Box<Option<bool>>,
    /// Specifies whether the Config Sync Repo is in "hierarchical" or "unstructured" mode.
    #[builder(into, default)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Box<Option<String>>,
    /// Set to `true` to stop syncing configurations for a single cluster. This field is only available on clusters using Config Sync [auto-upgrades](http://cloud/kubernetes-engine/enterprise/config-sync/docs/how-to/upgrade-config-sync#auto-upgrade-config) or on Config Sync version 1.20.0 or later. Defaults: `false`.
    #[builder(into, default)]
    #[serde(rename = "stopSyncing")]
    pub r#stop_syncing: Box<Option<bool>>,
}
