#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureSpecClusterupgradeGkeUpgradeOverrideUpgrade {
    /// Name of the upgrade, e.g., "k8s_control_plane". It should be a valid upgrade name. It must not exceet 99 characters.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Version of the upgrade, e.g., "1.22.1-gke.100". It should be a valid version. It must not exceet 99 characters.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
