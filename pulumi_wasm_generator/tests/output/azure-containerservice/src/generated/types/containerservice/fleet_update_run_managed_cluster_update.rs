#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetUpdateRunManagedClusterUpdate {
    /// A `node_image_selection` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "nodeImageSelection")]
    pub r#node_image_selection: Box<Option<super::super::types::containerservice::FleetUpdateRunManagedClusterUpdateNodeImageSelection>>,
    /// A `upgrade` block as defined below.
    #[builder(into)]
    #[serde(rename = "upgrade")]
    pub r#upgrade: Box<super::super::types::containerservice::FleetUpdateRunManagedClusterUpdateUpgrade>,
}
