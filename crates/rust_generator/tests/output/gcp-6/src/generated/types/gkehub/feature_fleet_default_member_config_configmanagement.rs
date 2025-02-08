#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigConfigmanagement {
    /// ConfigSync configuration for the cluster
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "configSync")]
    pub r#config_sync: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagementConfigSync>>,
    /// Set this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades.
    /// Possible values are: `MANAGEMENT_UNSPECIFIED`, `MANAGEMENT_AUTOMATIC`, `MANAGEMENT_MANUAL`.
    #[builder(into, default)]
    #[serde(rename = "management")]
    pub r#management: Box<Option<String>>,
    /// Version of Config Sync installed
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
