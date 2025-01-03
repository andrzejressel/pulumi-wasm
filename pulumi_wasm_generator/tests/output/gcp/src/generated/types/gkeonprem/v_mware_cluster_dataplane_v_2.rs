#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterDataplaneV2 {
    /// Enable advanced networking which requires dataplane_v2_enabled to be set true.
    #[builder(into, default)]
    #[serde(rename = "advancedNetworking")]
    pub r#advanced_networking: Box<Option<bool>>,
    /// Enables Dataplane V2.
    #[builder(into, default)]
    #[serde(rename = "dataplaneV2Enabled")]
    pub r#dataplane_v_2_enabled: Box<Option<bool>>,
    /// Enable Dataplane V2 for clusters with Windows nodes.
    #[builder(into, default)]
    #[serde(rename = "windowsDataplaneV2Enabled")]
    pub r#windows_dataplane_v_2_enabled: Box<Option<bool>>,
}
