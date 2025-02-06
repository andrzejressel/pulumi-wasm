#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceNetworkConfig {
    /// A list of external networks authorized to access this instance. This
    /// field is only allowed to be set when `enable_public_ip` is set to
    /// true.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorizedExternalNetworks")]
    pub r#authorized_external_networks: Box<Option<Vec<super::super::types::alloydb::InstanceNetworkConfigAuthorizedExternalNetwork>>>,
    /// Enabling outbound public ip for the instance.
    #[builder(into, default)]
    #[serde(rename = "enableOutboundPublicIp")]
    pub r#enable_outbound_public_ip: Box<Option<bool>>,
    /// Enabling public ip for the instance. If a user wishes to disable this,
    /// please also clear the list of the authorized external networks set on
    /// the same instance.
    #[builder(into, default)]
    #[serde(rename = "enablePublicIp")]
    pub r#enable_public_ip: Box<Option<bool>>,
}
