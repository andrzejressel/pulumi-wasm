#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceInitialReplicaSet {
    /// A list of subnet IP addresses for the domain controllers in the initial replica set, typically two.
    #[builder(into, default)]
    #[serde(rename = "domainControllerIpAddresses")]
    pub r#domain_controller_ip_addresses: Box<Option<Vec<String>>>,
    /// The publicly routable IP address for the domain controllers in the initial replica set.
    #[builder(into, default)]
    #[serde(rename = "externalAccessIpAddress")]
    pub r#external_access_ip_address: Box<Option<String>>,
    /// A unique ID for the replica set.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Azure location where the Domain Service exists. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The current service status for the initial replica set.
    #[builder(into, default)]
    #[serde(rename = "serviceStatus")]
    pub r#service_status: Box<Option<String>>,
    /// The ID of the subnet in which to place the initial replica set. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
