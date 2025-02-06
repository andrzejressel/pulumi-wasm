#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureSpecClusterupgradeGkeUpgradeOverride {
    /// Post conditions to override for the specified upgrade.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postConditions")]
    pub r#post_conditions: Box<super::super::types::gkehub::FeatureSpecClusterupgradeGkeUpgradeOverridePostConditions>,
    /// Which upgrade to override.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "upgrade")]
    pub r#upgrade: Box<super::super::types::gkehub::FeatureSpecClusterupgradeGkeUpgradeOverrideUpgrade>,
}
