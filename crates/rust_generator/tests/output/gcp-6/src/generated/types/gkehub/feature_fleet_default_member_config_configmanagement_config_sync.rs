#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigConfigmanagementConfigSync {
    /// Enables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Git repo configuration for the cluster
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "git")]
    pub r#git: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncGit>>,
    /// OCI repo configuration for the cluster
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oci")]
    pub r#oci: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci>>,
    /// Set to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts.
    #[builder(into, default)]
    #[serde(rename = "preventDrift")]
    pub r#prevent_drift: Box<Option<bool>>,
    /// Specifies whether the Config Sync Repo is in hierarchical or unstructured mode
    #[builder(into, default)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Box<Option<String>>,
}
