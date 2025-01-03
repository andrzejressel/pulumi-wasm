/// The NetworkConnectivity Spoke resource
///
///
/// To get more information about Spoke, see:
///
/// * [API documentation](https://cloud.google.com/network-connectivity/docs/reference/networkconnectivity/rest/v1beta/projects.locations.spokes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/overview)
///
/// ## Example Usage
///
/// ### Network Connectivity Spoke Linked Vpc Network Basic
///
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: net
///       autoCreateSubnetworks: false
///   basicHub:
///     type: gcp:networkconnectivity:Hub
///     name: basic_hub
///     properties:
///       name: hub1
///       description: A sample hub
///       labels:
///         label-two: value-one
///   primary:
///     type: gcp:networkconnectivity:Spoke
///     properties:
///       name: spoke1
///       location: global
///       description: A sample spoke with a linked router appliance instance
///       labels:
///         label-one: value-one
///       hub: ${basicHub.id}
///       linkedVpcNetwork:
///         excludeExportRanges:
///           - 198.51.100.0/24
///           - 10.10.0.0/16
///         includeExportRanges:
///           - 198.51.100.0/23
///           - 10.0.0.0/8
///         uri: ${network.selfLink}
/// ```
/// ### Network Connectivity Spoke Router Appliance Basic
///
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: tf-test-network_55138
///       autoCreateSubnetworks: false
///   subnetwork:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: tf-test-subnet_37559
///       ipCidrRange: 10.0.0.0/28
///       region: us-central1
///       network: ${network.selfLink}
///   instance:
///     type: gcp:compute:Instance
///     properties:
///       name: tf-test-instance_91980
///       machineType: e2-medium
///       canIpForward: true
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: projects/debian-cloud/global/images/debian-10-buster-v20210817
///       networkInterfaces:
///         - subnetwork: ${subnetwork.name}
///           networkIp: 10.0.0.2
///           accessConfigs:
///             - networkTier: PREMIUM
///   basicHub:
///     type: gcp:networkconnectivity:Hub
///     name: basic_hub
///     properties:
///       name: tf-test-hub_37118
///       description: A sample hub
///       labels:
///         label-two: value-one
///   primary:
///     type: gcp:networkconnectivity:Spoke
///     properties:
///       name: tf-test-name_80332
///       location: us-central1
///       description: A sample spoke with a linked routher appliance instance
///       labels:
///         label-one: value-one
///       hub: ${basicHub.id}
///       linkedRouterApplianceInstances:
///         instances:
///           - virtualMachine: ${instance.selfLink}
///             ipAddress: 10.0.0.2
///         siteToSiteDataTransfer: true
///         includeImportRanges:
///           - ALL_IPV4_RANGES
/// ```
/// ### Network Connectivity Spoke Vpn Tunnel Basic
///
///
/// ```yaml
/// resources:
///   basicHub:
///     type: gcp:networkconnectivity:Hub
///     name: basic_hub
///     properties:
///       name: basic-hub1
///       description: A sample hub
///       labels:
///         label-two: value-one
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: basic-network
///       autoCreateSubnetworks: false
///   subnetwork:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: basic-subnetwork
///       ipCidrRange: 10.0.0.0/28
///       region: us-central1
///       network: ${network.selfLink}
///   gateway:
///     type: gcp:compute:HaVpnGateway
///     properties:
///       name: vpn-gateway
///       network: ${network.id}
///   externalVpnGw:
///     type: gcp:compute:ExternalVpnGateway
///     name: external_vpn_gw
///     properties:
///       name: external-vpn-gateway
///       redundancyType: SINGLE_IP_INTERNALLY_REDUNDANT
///       description: An externally managed VPN gateway
///       interfaces:
///         - id: 0
///           ipAddress: 8.8.8.8
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: external-vpn-gateway
///       region: us-central1
///       network: ${network.name}
///       bgp:
///         asn: 64514
///   tunnel1:
///     type: gcp:compute:VPNTunnel
///     properties:
///       name: tunnel1
///       region: us-central1
///       vpnGateway: ${gateway.id}
///       peerExternalGateway: ${externalVpnGw.id}
///       peerExternalGatewayInterface: 0
///       sharedSecret: a secret message
///       router: ${router.id}
///       vpnGatewayInterface: 0
///   tunnel2:
///     type: gcp:compute:VPNTunnel
///     properties:
///       name: tunnel2
///       region: us-central1
///       vpnGateway: ${gateway.id}
///       peerExternalGateway: ${externalVpnGw.id}
///       peerExternalGatewayInterface: 0
///       sharedSecret: a secret message
///       router: ' ${router.id}'
///       vpnGatewayInterface: 1
///   routerInterface1:
///     type: gcp:compute:RouterInterface
///     name: router_interface1
///     properties:
///       name: router-interface1
///       router: ${router.name}
///       region: us-central1
///       ipRange: 169.254.0.1/30
///       vpnTunnel: ${tunnel1.name}
///   routerPeer1:
///     type: gcp:compute:RouterPeer
///     name: router_peer1
///     properties:
///       name: router-peer1
///       router: ${router.name}
///       region: us-central1
///       peerIpAddress: 169.254.0.2
///       peerAsn: 64515
///       advertisedRoutePriority: 100
///       interface: ${routerInterface1.name}
///   routerInterface2:
///     type: gcp:compute:RouterInterface
///     name: router_interface2
///     properties:
///       name: router-interface2
///       router: ${router.name}
///       region: us-central1
///       ipRange: 169.254.1.1/30
///       vpnTunnel: ${tunnel2.name}
///   routerPeer2:
///     type: gcp:compute:RouterPeer
///     name: router_peer2
///     properties:
///       name: router-peer2
///       router: ${router.name}
///       region: us-central1
///       peerIpAddress: 169.254.1.2
///       peerAsn: 64515
///       advertisedRoutePriority: 100
///       interface: ${routerInterface2.name}
///   tunnel1Spoke:
///     type: gcp:networkconnectivity:Spoke
///     name: tunnel1
///     properties:
///       name: vpn-tunnel-1-spoke
///       location: us-central1
///       description: A sample spoke with a linked VPN Tunnel
///       labels:
///         label-one: value-one
///       hub: ${basicHub.id}
///       linkedVpnTunnels:
///         uris:
///           - ${tunnel1.selfLink}
///         siteToSiteDataTransfer: true
///         includeImportRanges:
///           - ALL_IPV4_RANGES
///   tunnel2Spoke:
///     type: gcp:networkconnectivity:Spoke
///     name: tunnel2
///     properties:
///       name: vpn-tunnel-2-spoke
///       location: us-central1
///       description: A sample spoke with a linked VPN Tunnel
///       labels:
///         label-one: value-one
///       hub: ${basicHub.id}
///       linkedVpnTunnels:
///         uris:
///           - ${tunnel2.selfLink}
///         siteToSiteDataTransfer: true
///         includeImportRanges:
///           - ALL_IPV4_RANGES
/// ```
/// ### Network Connectivity Spoke Interconnect Attachment Basic
///
///
/// ```yaml
/// resources:
///   basicHub:
///     type: gcp:networkconnectivity:Hub
///     name: basic_hub
///     properties:
///       name: basic-hub1
///       description: A sample hub
///       labels:
///         label-two: value-one
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: basic-network
///       autoCreateSubnetworks: false
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: external-vpn-gateway
///       region: us-central1
///       network: ${network.name}
///       bgp:
///         asn: 16550
///   interconnect-attachment:
///     type: gcp:compute:InterconnectAttachment
///     properties:
///       name: partner-interconnect1
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_1
///       type: PARTNER
///       router: ${router.id}
///       mtu: 1500
///       region: us-central1
///   primary:
///     type: gcp:networkconnectivity:Spoke
///     properties:
///       name: interconnect-attachment-spoke
///       location: us-central1
///       description: A sample spoke with a linked Interconnect Attachment
///       labels:
///         label-one: value-one
///       hub: ${basicHub.id}
///       linkedInterconnectAttachments:
///         uris:
///           - ${["interconnect-attachment"].selfLink}
///         siteToSiteDataTransfer: true
///         includeImportRanges:
///           - ALL_IPV4_RANGES
/// ```
/// ### Network Connectivity Spoke Linked Producer Vpc Network Basic
///
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: net-spoke
///       autoCreateSubnetworks: false
///   address:
///     type: gcp:compute:GlobalAddress
///     properties:
///       name: test-address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${network.id}
///   peering:
///     type: gcp:servicenetworking:Connection
///     properties:
///       network: ${network.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${address.name}
///   basicHub:
///     type: gcp:networkconnectivity:Hub
///     name: basic_hub
///     properties:
///       name: hub-basic
///   linkedVpcSpoke:
///     type: gcp:networkconnectivity:Spoke
///     name: linked_vpc_spoke
///     properties:
///       name: vpc-spoke
///       location: global
///       hub: ${basicHub.id}
///       linkedVpcNetwork:
///         uri: ${network.selfLink}
///   primary:
///     type: gcp:networkconnectivity:Spoke
///     properties:
///       name: producer-spoke
///       location: global
///       description: A sample spoke with a linked router appliance instance
///       labels:
///         label-one: value-one
///       hub: ${basicHub.id}
///       linkedProducerVpcNetwork:
///         network: ${network.name}
///         peering: ${peering.peering}
///         excludeExportRanges:
///           - 198.51.100.0/24
///           - 10.10.0.0/16
///     options:
///       dependsOn:
///         - ${linkedVpcSpoke}
/// ```
///
/// ## Import
///
/// Spoke can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/spokes/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Spoke can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/spoke:Spoke default projects/{{project}}/locations/{{location}}/spokes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/spoke:Spoke default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/spoke:Spoke default {{location}}/{{name}}
/// ```
///
pub mod spoke {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpokeArgs {
        /// An optional description of the spoke.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Immutable. The URI of the hub that this spoke is attached to.
        #[builder(into)]
        pub hub: pulumi_wasm_rust::Output<String>,
        /// Optional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A collection of VLAN attachment resources. These resources should be redundant attachments that all advertise the same prefixes to Google Cloud. Alternatively, in active/passive configurations, all attachments should be capable of advertising the same prefixes.
        /// Structure is documented below.
        #[builder(into, default)]
        pub linked_interconnect_attachments: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::SpokeLinkedInterconnectAttachments,
            >,
        >,
        /// Producer VPC network that is associated with the spoke.
        /// Structure is documented below.
        #[builder(into, default)]
        pub linked_producer_vpc_network: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::SpokeLinkedProducerVpcNetwork,
            >,
        >,
        /// The URIs of linked Router appliance resources
        /// Structure is documented below.
        #[builder(into, default)]
        pub linked_router_appliance_instances: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::SpokeLinkedRouterApplianceInstances,
            >,
        >,
        /// VPC network that is associated with the spoke.
        /// Structure is documented below.
        #[builder(into, default)]
        pub linked_vpc_network: pulumi_wasm_rust::Output<
            Option<super::super::types::networkconnectivity::SpokeLinkedVpcNetwork>,
        >,
        /// The URIs of linked VPN tunnel resources
        /// Structure is documented below.
        #[builder(into, default)]
        pub linked_vpn_tunnels: pulumi_wasm_rust::Output<
            Option<super::super::types::networkconnectivity::SpokeLinkedVpnTunnels>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Immutable. The name of the spoke. Spoke names must be unique.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SpokeResult {
        /// Output only. The time the spoke was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An optional description of the spoke.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. The URI of the hub that this spoke is attached to.
        pub hub: pulumi_wasm_rust::Output<String>,
        /// Optional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A collection of VLAN attachment resources. These resources should be redundant attachments that all advertise the same prefixes to Google Cloud. Alternatively, in active/passive configurations, all attachments should be capable of advertising the same prefixes.
        /// Structure is documented below.
        pub linked_interconnect_attachments: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::SpokeLinkedInterconnectAttachments,
            >,
        >,
        /// Producer VPC network that is associated with the spoke.
        /// Structure is documented below.
        pub linked_producer_vpc_network: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::SpokeLinkedProducerVpcNetwork,
            >,
        >,
        /// The URIs of linked Router appliance resources
        /// Structure is documented below.
        pub linked_router_appliance_instances: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkconnectivity::SpokeLinkedRouterApplianceInstances,
            >,
        >,
        /// VPC network that is associated with the spoke.
        /// Structure is documented below.
        pub linked_vpc_network: pulumi_wasm_rust::Output<
            Option<super::super::types::networkconnectivity::SpokeLinkedVpcNetwork>,
        >,
        /// The URIs of linked VPN tunnel resources
        /// Structure is documented below.
        pub linked_vpn_tunnels: pulumi_wasm_rust::Output<
            Option<super::super::types::networkconnectivity::SpokeLinkedVpnTunnels>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Immutable. The name of the spoke. Spoke names must be unique.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The current lifecycle state of this spoke.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different unique_id.
        pub unique_id: pulumi_wasm_rust::Output<String>,
        /// Output only. The time the spoke was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SpokeArgs) -> SpokeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let hub_binding = args.hub.get_inner();
        let labels_binding = args.labels.get_inner();
        let linked_interconnect_attachments_binding = args
            .linked_interconnect_attachments
            .get_inner();
        let linked_producer_vpc_network_binding = args
            .linked_producer_vpc_network
            .get_inner();
        let linked_router_appliance_instances_binding = args
            .linked_router_appliance_instances
            .get_inner();
        let linked_vpc_network_binding = args.linked_vpc_network.get_inner();
        let linked_vpn_tunnels_binding = args.linked_vpn_tunnels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/spoke:Spoke".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hub".into(),
                    value: &hub_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "linkedInterconnectAttachments".into(),
                    value: &linked_interconnect_attachments_binding,
                },
                register_interface::ObjectField {
                    name: "linkedProducerVpcNetwork".into(),
                    value: &linked_producer_vpc_network_binding,
                },
                register_interface::ObjectField {
                    name: "linkedRouterApplianceInstances".into(),
                    value: &linked_router_appliance_instances_binding,
                },
                register_interface::ObjectField {
                    name: "linkedVpcNetwork".into(),
                    value: &linked_vpc_network_binding,
                },
                register_interface::ObjectField {
                    name: "linkedVpnTunnels".into(),
                    value: &linked_vpn_tunnels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
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
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "hub".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "linkedInterconnectAttachments".into(),
                },
                register_interface::ResultField {
                    name: "linkedProducerVpcNetwork".into(),
                },
                register_interface::ResultField {
                    name: "linkedRouterApplianceInstances".into(),
                },
                register_interface::ResultField {
                    name: "linkedVpcNetwork".into(),
                },
                register_interface::ResultField {
                    name: "linkedVpnTunnels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uniqueId".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpokeResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            hub: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hub").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            linked_interconnect_attachments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedInterconnectAttachments").unwrap(),
            ),
            linked_producer_vpc_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedProducerVpcNetwork").unwrap(),
            ),
            linked_router_appliance_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedRouterApplianceInstances").unwrap(),
            ),
            linked_vpc_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedVpcNetwork").unwrap(),
            ),
            linked_vpn_tunnels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedVpnTunnels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            unique_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueId").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
