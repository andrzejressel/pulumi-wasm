/// Represents a network peering resource. Network peerings are global resources.
///
///
/// To get more information about NetworkPeering, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/networks/addPeering)
///
/// ## Example Usage
///
/// ### Vmware Engine Network Peering Ven
///
///
/// ```yaml
/// resources:
///   network-peering-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: default-np-nw
///       location: global
///       type: STANDARD
///   network-peering-peer-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: peer-np-nw
///       location: global
///       type: STANDARD
///   vmw-engine-network-peering:
///     type: gcp:vmwareengine:NetworkPeering
///     properties:
///       name: sample-network-peering
///       description: Sample description
///       vmwareEngineNetwork: ${["network-peering-nw"].id}
///       peerNetwork: ${["network-peering-peer-nw"].id}
///       peerNetworkType: VMWARE_ENGINE_NETWORK
///       exportCustomRoutes: false
///       importCustomRoutes: false
///       exportCustomRoutesWithPublicIp: false
///       importCustomRoutesWithPublicIp: false
/// ```
/// ### Vmware Engine Network Peering Standard
///
///
/// ```yaml
/// resources:
///   network-peering-vpc:
///     type: gcp:compute:Network
///     properties:
///       name: default-vpc
///   network-peering-standard-nw:
///     type: gcp:vmwareengine:Network
///     properties:
///       name: default-standard-nw-np
///       location: global
///       type: STANDARD
///   vmw-engine-network-peering:
///     type: gcp:vmwareengine:NetworkPeering
///     properties:
///       name: sample-network-peering
///       description: Sample description
///       peerNetwork: ${["network-peering-vpc"].id}
///       peerNetworkType: STANDARD
///       vmwareEngineNetwork: ${["network-peering-standard-nw"].id}
/// ```
///
/// ## Import
///
/// NetworkPeering can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/networkPeerings/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, NetworkPeering can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/networkPeering:NetworkPeering default projects/{{project}}/locations/global/networkPeerings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/networkPeering:NetworkPeering default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vmwareengine/networkPeering:NetworkPeering default {{name}}
/// ```
///
pub mod network_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkPeeringArgs {
        /// User-provided description for this network peering.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// True if custom routes are exported to the peered network; false otherwise.
        #[builder(into, default)]
        pub export_custom_routes: pulumi_wasm_rust::Output<Option<bool>>,
        /// True if all subnet routes with a public IP address range are exported; false otherwise.
        #[builder(into, default)]
        pub export_custom_routes_with_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// True if custom routes are imported from the peered network; false otherwise.
        #[builder(into, default)]
        pub import_custom_routes: pulumi_wasm_rust::Output<Option<bool>>,
        /// True if custom routes are imported from the peered network; false otherwise.
        #[builder(into, default)]
        pub import_custom_routes_with_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Network Peering.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the network to peer with a standard VMware Engine network.
        /// The provided network can be a consumer VPC network or another standard VMware Engine network.
        #[builder(into)]
        pub peer_network: pulumi_wasm_rust::Output<String>,
        /// The type of the network to peer with the VMware Engine network.
        /// Possible values are: `STANDARD`, `VMWARE_ENGINE_NETWORK`, `PRIVATE_SERVICES_ACCESS`, `NETAPP_CLOUD_VOLUMES`, `THIRD_PARTY_SERVICE`, `DELL_POWERSCALE`.
        #[builder(into)]
        pub peer_network_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the VMware Engine network. Specify the name in the following form:
        /// projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
        /// can either be a project number or a project ID.
        #[builder(into)]
        pub vmware_engine_network: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkPeeringResult {
        /// Creation time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-provided description for this network peering.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// True if custom routes are exported to the peered network; false otherwise.
        pub export_custom_routes: pulumi_wasm_rust::Output<Option<bool>>,
        /// True if all subnet routes with a public IP address range are exported; false otherwise.
        pub export_custom_routes_with_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// True if custom routes are imported from the peered network; false otherwise.
        pub import_custom_routes: pulumi_wasm_rust::Output<Option<bool>>,
        /// True if custom routes are imported from the peered network; false otherwise.
        pub import_custom_routes_with_public_ip: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Network Peering.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the network to peer with a standard VMware Engine network.
        /// The provided network can be a consumer VPC network or another standard VMware Engine network.
        pub peer_network: pulumi_wasm_rust::Output<String>,
        /// The type of the network to peer with the VMware Engine network.
        /// Possible values are: `STANDARD`, `VMWARE_ENGINE_NETWORK`, `PRIVATE_SERVICES_ACCESS`, `NETAPP_CLOUD_VOLUMES`, `THIRD_PARTY_SERVICE`, `DELL_POWERSCALE`.
        pub peer_network_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// State of the network peering.
        /// This field has a value of 'ACTIVE' when there's a matching configuration in the peer network.
        /// New values may be added to this enum when appropriate.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Details about the current state of the network peering.
        pub state_details: pulumi_wasm_rust::Output<String>,
        /// System-generated unique identifier for the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Last updated time of this resource.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the VMware Engine network. Specify the name in the following form:
        /// projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
        /// can either be a project number or a project ID.
        pub vmware_engine_network: pulumi_wasm_rust::Output<String>,
        /// The canonical name of the VMware Engine network in the form:
        /// projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
        pub vmware_engine_network_canonical: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkPeeringArgs) -> NetworkPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let export_custom_routes_binding = args.export_custom_routes.get_inner();
        let export_custom_routes_with_public_ip_binding = args
            .export_custom_routes_with_public_ip
            .get_inner();
        let import_custom_routes_binding = args.import_custom_routes.get_inner();
        let import_custom_routes_with_public_ip_binding = args
            .import_custom_routes_with_public_ip
            .get_inner();
        let name_binding = args.name.get_inner();
        let peer_network_binding = args.peer_network.get_inner();
        let peer_network_type_binding = args.peer_network_type.get_inner();
        let project_binding = args.project.get_inner();
        let vmware_engine_network_binding = args.vmware_engine_network.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vmwareengine/networkPeering:NetworkPeering".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "exportCustomRoutes".into(),
                    value: &export_custom_routes_binding,
                },
                register_interface::ObjectField {
                    name: "exportCustomRoutesWithPublicIp".into(),
                    value: &export_custom_routes_with_public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "importCustomRoutes".into(),
                    value: &import_custom_routes_binding,
                },
                register_interface::ObjectField {
                    name: "importCustomRoutesWithPublicIp".into(),
                    value: &import_custom_routes_with_public_ip_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "peerNetwork".into(),
                    value: &peer_network_binding,
                },
                register_interface::ObjectField {
                    name: "peerNetworkType".into(),
                    value: &peer_network_type_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "vmwareEngineNetwork".into(),
                    value: &vmware_engine_network_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "exportCustomRoutes".into(),
                },
                register_interface::ResultField {
                    name: "exportCustomRoutesWithPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "importCustomRoutes".into(),
                },
                register_interface::ResultField {
                    name: "importCustomRoutesWithPublicIp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peerNetwork".into(),
                },
                register_interface::ResultField {
                    name: "peerNetworkType".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateDetails".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetwork".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetworkCanonical".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkPeeringResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            export_custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportCustomRoutes").unwrap(),
            ),
            export_custom_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportCustomRoutesWithPublicIp").unwrap(),
            ),
            import_custom_routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importCustomRoutes").unwrap(),
            ),
            import_custom_routes_with_public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importCustomRoutesWithPublicIp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peer_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerNetwork").unwrap(),
            ),
            peer_network_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerNetworkType").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateDetails").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vmware_engine_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetwork").unwrap(),
            ),
            vmware_engine_network_canonical: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetworkCanonical").unwrap(),
            ),
        }
    }
}
