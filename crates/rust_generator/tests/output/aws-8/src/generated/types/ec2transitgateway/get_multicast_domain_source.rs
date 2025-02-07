#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMulticastDomainSource {
    /// The IP address assigned to the transit gateway multicast group.
    #[builder(into)]
    #[serde(rename = "groupIpAddress")]
    pub r#group_ip_address: Box<String>,
    /// The group members' network interface ID.
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<String>,
}
