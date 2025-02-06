#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntentVirtualSwitchConfigurationOverride {
    /// Specifies the IoV enable status for Virtual Switch. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "enableIov")]
    pub r#enable_iov: Box<Option<String>>,
    /// Specifies the load balancing algorithm for Virtual Switch. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "loadBalancingAlgorithm")]
    pub r#load_balancing_algorithm: Box<Option<String>>,
}
