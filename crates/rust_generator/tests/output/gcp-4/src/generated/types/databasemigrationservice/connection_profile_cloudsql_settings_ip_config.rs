#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileCloudsqlSettingsIpConfig {
    /// The list of external networks that are allowed to connect to the instance using the IP.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorizedNetworks")]
    pub r#authorized_networks: Box<Option<Vec<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettingsIpConfigAuthorizedNetwork>>>,
    /// Whether the instance should be assigned an IPv4 address or not.
    #[builder(into, default)]
    #[serde(rename = "enableIpv4")]
    pub r#enable_ipv_4: Box<Option<bool>>,
    /// The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.
    /// This setting can be updated, but it cannot be removed after it is set.
    #[builder(into, default)]
    #[serde(rename = "privateNetwork")]
    pub r#private_network: Box<Option<String>>,
    /// Whether SSL connections over IP should be enforced or not.
    #[builder(into, default)]
    #[serde(rename = "requireSsl")]
    pub r#require_ssl: Box<Option<bool>>,
}
