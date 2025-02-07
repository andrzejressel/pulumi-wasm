#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceReplicaSet {
    /// A list of subnet IP addresses for the domain controllers in the replica set, typically two.
    #[builder(into)]
    #[serde(rename = "domainControllerIpAddresses")]
    pub r#domain_controller_ip_addresses: Box<Vec<String>>,
    /// The publicly routable IP address for the domain controllers in the replica set.
    #[builder(into)]
    #[serde(rename = "externalAccessIpAddress")]
    pub r#external_access_ip_address: Box<String>,
    /// The ID of the Domain Service.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The Azure location in which the replica set resides.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The current service status for the replica set.
    #[builder(into)]
    #[serde(rename = "serviceStatus")]
    pub r#service_status: Box<String>,
    /// The ID of the subnet in which the replica set resides.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
