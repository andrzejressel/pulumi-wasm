#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterStorage {
    /// Whether or not to deploy vSphere CSI components in the VMware User Cluster.
    /// Enabled by default.
    #[builder(into)]
    #[serde(rename = "vsphereCsiDisabled")]
    pub r#vsphere_csi_disabled: Box<bool>,
}
