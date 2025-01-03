/// A Region network endpoint represents a IP address/FQDN and port combination that is
/// part of a specific network endpoint group (NEG).
///
/// > **NOTE**: Network endpoints cannot be created outside of a network endpoint group.
///
///
/// To get more information about RegionNetworkEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/regionNetworkEndpointGroups)
/// * How-to Guides
///     * [Internet NEGs Official Documentation](https://cloud.google.com/load-balancing/docs/negs/internet-neg-concepts)
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/negs/)
///
/// ## Example Usage
///
/// ### Region Network Endpoint Internet Ip Port
///
///
/// ```yaml
/// resources:
///   region-internet-ip-port-endpoint:
///     type: gcp:compute:RegionNetworkEndpoint
///     properties:
///       regionNetworkEndpointGroup: ${group.name}
///       region: us-central1
///       ipAddress: 8.8.8.8
///       port: 443
///   group:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     properties:
///       name: ip-port-neg
///       network: ${default.id}
///       region: us-central1
///       networkEndpointType: INTERNET_IP_PORT
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: network
///       autoCreateSubnetworks: false
/// ```
/// ### Region Network Endpoint Internet Fqdn Port
///
///
/// ```yaml
/// resources:
///   region-internet-fqdn-port-endpoint:
///     type: gcp:compute:RegionNetworkEndpoint
///     properties:
///       regionNetworkEndpointGroup: ${group.name}
///       region: us-central1
///       fqdn: backend.example.com
///       port: 443
///   group:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     properties:
///       name: fqdn-port-neg
///       network: ${default.id}
///       region: us-central1
///       networkEndpointType: INTERNET_FQDN_PORT
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: network
///       autoCreateSubnetworks: false
/// ```
/// ### Region Network Endpoint Portmap
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: network
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: subnetwork
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///       network: ${default.id}
///   defaultRegionNetworkEndpointGroup:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     name: default
///     properties:
///       name: portmap-neg
///       region: us-central1
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       networkEndpointType: GCE_VM_IP_PORTMAP
///   regionNetworkEndpointPortmap:
///     type: gcp:compute:RegionNetworkEndpoint
///     name: region_network_endpoint_portmap
///     properties:
///       regionNetworkEndpointGroup: ${defaultRegionNetworkEndpointGroup.name}
///       region: us-central1
///       instance: ${defaultInstance.selfLink}
///       port: 80
///       ipAddress: ${defaultInstance.networkInterfaces[0].networkIp}
///       clientDestinationPort: 8080
///   defaultInstance:
///     type: gcp:compute:Instance
///     name: default
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           subnetwork: ${defaultSubnetwork.id}
///       name: instance
///       machineType: e2-medium
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: ${myImage.selfLink}
/// variables:
///   myImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
///
/// ## Import
///
/// RegionNetworkEndpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/networkEndpointGroups/{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// * `{{project}}/{{region}}/{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// * `{{region}}/{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// * `{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}`
///
/// When using the `pulumi import` command, RegionNetworkEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpoint:RegionNetworkEndpoint default projects/{{project}}/regions/{{region}}/networkEndpointGroups/{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpoint:RegionNetworkEndpoint default {{project}}/{{region}}/{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpoint:RegionNetworkEndpoint default {{region}}/{{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkEndpoint:RegionNetworkEndpoint default {{region_network_endpoint_group}}/{{ip_address}}/{{fqdn}}/{{port}}
/// ```
///
pub mod region_network_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionNetworkEndpointArgs {
        /// Client destination port for the `GCE_VM_IP_PORTMAP` NEG.
        #[builder(into, default)]
        pub client_destination_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Fully qualified domain name of network endpoint.
        /// This can only be specified when network_endpoint_type of the NEG is INTERNET_FQDN_PORT.
        #[builder(into, default)]
        pub fqdn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for a specific VM instance that the IP address belongs to.
        /// This is required for network endpoints of type GCE_VM_IP_PORTMAP.
        #[builder(into, default)]
        pub instance: pulumi_wasm_rust::Output<Option<String>>,
        /// IPv4 address external endpoint.
        /// This can only be specified when network_endpoint_type of the NEG is INTERNET_IP_PORT.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// Port number of network endpoint.
        #[builder(into)]
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Region where the containing network endpoint group is located.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The network endpoint group this endpoint is part of.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub region_network_endpoint_group: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RegionNetworkEndpointResult {
        /// Client destination port for the `GCE_VM_IP_PORTMAP` NEG.
        pub client_destination_port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Fully qualified domain name of network endpoint.
        /// This can only be specified when network_endpoint_type of the NEG is INTERNET_FQDN_PORT.
        pub fqdn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for a specific VM instance that the IP address belongs to.
        /// This is required for network endpoints of type GCE_VM_IP_PORTMAP.
        pub instance: pulumi_wasm_rust::Output<Option<String>>,
        /// IPv4 address external endpoint.
        /// This can only be specified when network_endpoint_type of the NEG is INTERNET_IP_PORT.
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub network_endpoint_id: pulumi_wasm_rust::Output<i32>,
        /// Port number of network endpoint.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where the containing network endpoint group is located.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The network endpoint group this endpoint is part of.
        ///
        ///
        /// - - -
        pub region_network_endpoint_group: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RegionNetworkEndpointArgs,
    ) -> RegionNetworkEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_destination_port_binding = args.client_destination_port.get_inner();
        let fqdn_binding = args.fqdn.get_inner();
        let instance_binding = args.instance.get_inner();
        let ip_address_binding = args.ip_address.get_inner();
        let port_binding = args.port.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let region_network_endpoint_group_binding = args
            .region_network_endpoint_group
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionNetworkEndpoint:RegionNetworkEndpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientDestinationPort".into(),
                    value: &client_destination_port_binding,
                },
                register_interface::ObjectField {
                    name: "fqdn".into(),
                    value: &fqdn_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "regionNetworkEndpointGroup".into(),
                    value: &region_network_endpoint_group_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientDestinationPort".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "networkEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "regionNetworkEndpointGroup".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionNetworkEndpointResult {
            client_destination_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientDestinationPort").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            network_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkEndpointId").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            region_network_endpoint_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionNetworkEndpointGroup").unwrap(),
            ),
        }
    }
}
