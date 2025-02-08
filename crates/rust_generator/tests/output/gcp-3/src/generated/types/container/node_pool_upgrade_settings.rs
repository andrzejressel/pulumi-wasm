#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NodePoolUpgradeSettings {
    /// The settings to adjust [blue green upgrades](https://cloud.google.com/kubernetes-engine/docs/concepts/node-pool-upgrade-strategies#blue-green-upgrade-strategy).
    /// Structure is documented below
    #[builder(into, default)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Box<Option<super::super::types::container::NodePoolUpgradeSettingsBlueGreenSettings>>,
    /// The number of additional nodes that can be added to the node pool during
    /// an upgrade. Increasing `max_surge` raises the number of nodes that can be upgraded simultaneously.
    /// Can be set to 0 or greater.
    #[builder(into, default)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<Option<i32>>,
    /// The number of nodes that can be simultaneously unavailable during
    /// an upgrade. Increasing `max_unavailable` raises the number of nodes that can be upgraded in
    /// parallel. Can be set to 0 or greater.
    /// 
    /// `max_surge` and `max_unavailable` must not be negative and at least one of them must be greater than zero.
    #[builder(into, default)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Box<Option<i32>>,
    /// The upgrade strategy to be used for upgrading the nodes.
    #[builder(into, default)]
    #[serde(rename = "strategy")]
    pub r#strategy: Box<Option<String>>,
}
