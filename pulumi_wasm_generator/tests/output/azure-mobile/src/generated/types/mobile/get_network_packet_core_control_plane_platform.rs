#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkPacketCoreControlPlanePlatform {
    /// The ID of Azure Arc connected cluster where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "arcKubernetesClusterId")]
    pub r#arc_kubernetes_cluster_id: Box<String>,
    /// The ID of Azure Arc custom location where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "customLocationId")]
    pub r#custom_location_id: Box<String>,
    /// The ID of Azure Stack Edge device where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "edgeDeviceId")]
    pub r#edge_device_id: Box<String>,
    /// The ID of Azure Stack HCI cluster where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "stackHciClusterId")]
    pub r#stack_hci_cluster_id: Box<String>,
    /// The platform type where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
