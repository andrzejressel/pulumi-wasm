#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PerInstanceConfigPreservedState {
    /// Stateful disks for the instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "disks")]
    pub r#disks: Box<Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateDisk>>>,
    /// Preserved external IPs defined for this instance. This map is keyed with the name of the network interface.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "externalIps")]
    pub r#external_ips: Box<Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateExternalIp>>>,
    /// Preserved internal IPs defined for this instance. This map is keyed with the name of the network interface.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "internalIps")]
    pub r#internal_ips: Box<Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateInternalIp>>>,
    /// Preserved metadata defined for this instance. This is a list of key->value pairs.
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
}
