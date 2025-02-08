#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VMwareClusterUpgradePolicy {
    /// Controls whether the upgrade applies to the control plane only.
    #[builder(into, default)]
    #[serde(rename = "controlPlaneOnly")]
    pub r#control_plane_only: Box<Option<bool>>,
}
