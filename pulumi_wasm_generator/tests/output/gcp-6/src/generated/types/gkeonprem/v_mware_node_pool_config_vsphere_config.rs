#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareNodePoolConfigVsphereConfig {
    /// The name of the vCenter datastore. Inherited from the user cluster.
    #[builder(into, default)]
    #[serde(rename = "datastore")]
    pub r#datastore: Box<Option<String>>,
    /// Vsphere host groups to apply to all VMs in the node pool
    #[builder(into, default)]
    #[serde(rename = "hostGroups")]
    pub r#host_groups: Box<Option<Vec<String>>>,
    /// Tags to apply to VMs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<super::super::types::gkeonprem::VMwareNodePoolConfigVsphereConfigTag>>>,
}
