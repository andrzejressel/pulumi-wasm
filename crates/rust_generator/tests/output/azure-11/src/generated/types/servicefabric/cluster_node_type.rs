#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodeType {
    /// A `application_ports` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "applicationPorts")]
    pub r#application_ports: Box<Option<super::super::types::servicefabric::ClusterNodeTypeApplicationPorts>>,
    /// The capacity tags applied to the nodes in the node type, the cluster resource manager uses these tags to understand how much resource a node has.
    #[builder(into, default)]
    #[serde(rename = "capacities")]
    pub r#capacities: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Port used for the Client Endpoint for this Node Type.
    #[builder(into)]
    #[serde(rename = "clientEndpointPort")]
    pub r#client_endpoint_port: Box<i32>,
    /// The Durability Level for this Node Type. Possible values include `Bronze`, `Gold` and `Silver`. Defaults to `Bronze`.
    #[builder(into, default)]
    #[serde(rename = "durabilityLevel")]
    pub r#durability_level: Box<Option<String>>,
    /// A `ephemeral_ports` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "ephemeralPorts")]
    pub r#ephemeral_ports: Box<Option<super::super::types::servicefabric::ClusterNodeTypeEphemeralPorts>>,
    /// The Port used for the HTTP Endpoint for this Node Type.
    #[builder(into)]
    #[serde(rename = "httpEndpointPort")]
    pub r#http_endpoint_port: Box<i32>,
    /// The number of nodes for this Node Type.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<i32>,
    /// Is this the Primary Node Type?
    #[builder(into)]
    #[serde(rename = "isPrimary")]
    pub r#is_primary: Box<bool>,
    /// Should this node type run only stateless services?
    #[builder(into, default)]
    #[serde(rename = "isStateless")]
    pub r#is_stateless: Box<Option<bool>>,
    /// Does this node type span availability zones?
    #[builder(into, default)]
    #[serde(rename = "multipleAvailabilityZones")]
    pub r#multiple_availability_zones: Box<Option<bool>>,
    /// The name of the Node Type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The placement tags applied to nodes in the node type, which can be used to indicate where certain services (workload) should run.
    #[builder(into, default)]
    #[serde(rename = "placementProperties")]
    pub r#placement_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Port used for the Reverse Proxy Endpoint for this Node Type. Changing this will upgrade the cluster.
    #[builder(into, default)]
    #[serde(rename = "reverseProxyEndpointPort")]
    pub r#reverse_proxy_endpoint_port: Box<Option<i32>>,
}
