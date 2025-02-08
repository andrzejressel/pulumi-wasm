#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HciDeploymentSettingScaleUnitHostNetworkIntent {
    /// A `adapter_property_override` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "adapterPropertyOverride")]
    pub r#adapter_property_override: Box<Option<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentAdapterPropertyOverride>>,
    /// Whether to override adapter properties. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "adapterPropertyOverrideEnabled")]
    pub r#adapter_property_override_enabled: Box<Option<bool>>,
    /// Specifies a list of ID of network interfaces used for the network intent. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "adapters")]
    pub r#adapters: Box<Vec<String>>,
    /// Specifies the name of the intent. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `qos_policy_override` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "qosPolicyOverride")]
    pub r#qos_policy_override: Box<Option<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentQosPolicyOverride>>,
    /// Whether to override QoS policy. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "qosPolicyOverrideEnabled")]
    pub r#qos_policy_override_enabled: Box<Option<bool>>,
    /// Specifies a list of network traffic types. Possible values are `Compute`, `Storage`, `Management`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "trafficTypes")]
    pub r#traffic_types: Box<Vec<String>>,
    /// A `virtual_switch_configuration_override` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualSwitchConfigurationOverride")]
    pub r#virtual_switch_configuration_override: Box<Option<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntentVirtualSwitchConfigurationOverride>>,
    /// Whether to override virtual switch configuration. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualSwitchConfigurationOverrideEnabled")]
    pub r#virtual_switch_configuration_override_enabled: Box<Option<bool>>,
}
