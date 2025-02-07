#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride {
    /// Specifies the percentage of the allocated storage traffic bandwidth. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "bandwidthPercentageSmb")]
    pub r#bandwidth_percentage_smb: Box<Option<String>>,
    /// Specifies the Cluster traffic priority. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "priorityValue8021ActionCluster")]
    pub r#priority_value_8021_action_cluster: Box<Option<String>>,
    /// Specifies the Priority Flow Control where Data Center Bridging (DCB) is used. This parameter should only be modified based on your OEM guidance. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "priorityValue8021ActionSmb")]
    pub r#priority_value_8021_action_smb: Box<Option<String>>,
}
