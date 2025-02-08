#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride {
    /// The jumbo frame size of the adapter. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "jumboPacket")]
    pub r#jumbo_packet: Box<Option<String>>,
    /// The network direct of the adapter. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "networkDirect")]
    pub r#network_direct: Box<Option<String>>,
    /// The network direct technology of the adapter. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "networkDirectTechnology")]
    pub r#network_direct_technology: Box<Option<String>>,
}
