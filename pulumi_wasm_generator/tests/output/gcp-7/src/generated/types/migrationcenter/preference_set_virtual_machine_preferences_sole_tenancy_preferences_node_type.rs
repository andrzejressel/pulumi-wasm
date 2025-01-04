#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreferenceSetVirtualMachinePreferencesSoleTenancyPreferencesNodeType {
    /// Name of the Sole Tenant node. Consult https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes
    #[builder(into, default)]
    #[serde(rename = "nodeName")]
    pub r#node_name: Box<Option<String>>,
}
