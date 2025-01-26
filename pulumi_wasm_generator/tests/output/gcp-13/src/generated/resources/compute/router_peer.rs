/// BGP information that must be configured into the routing stack to
/// establish BGP peering. This information must specify the peer ASN
/// and either the interface name, IP address, or peer IP address.
/// Please refer to RFC4273.
///
///
/// To get more information about RouterBgpPeer, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routers)
/// * How-to Guides
///     * [Google Cloud Router](https://cloud.google.com/router/docs/)
///
/// ## Example Usage
///
/// ### Router Peer Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let peer = router_peer::create(
///         "peer",
///         RouterPeerArgs::builder()
///             .advertised_route_priority(100)
///             .interface("interface-1")
///             .name("my-router-peer")
///             .peer_asn(65513)
///             .region("us-central1")
///             .router("my-router")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Router Peer Disabled
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let peer = router_peer::create(
///         "peer",
///         RouterPeerArgs::builder()
///             .advertised_route_priority(100)
///             .enable(false)
///             .interface("interface-1")
///             .name("my-router-peer")
///             .peer_asn(65513)
///             .peer_ip_address("169.254.1.2")
///             .region("us-central1")
///             .router("my-router")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Router Peer Bfd
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let peer = router_peer::create(
///         "peer",
///         RouterPeerArgs::builder()
///             .advertised_route_priority(100)
///             .bfd(
///                 RouterPeerBfd::builder()
///                     .minReceiveInterval(1000)
///                     .minTransmitInterval(1000)
///                     .multiplier(5)
///                     .sessionInitializationMode("ACTIVE")
///                     .build_struct(),
///             )
///             .interface("interface-1")
///             .name("my-router-peer")
///             .peer_asn(65513)
///             .peer_ip_address("169.254.1.2")
///             .region("us-central1")
///             .router("my-router")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Router Peer Router Appliance
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let addrIntf = address::create(
///         "addrIntf",
///         AddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("my-router-addr-intf")
///             .region("${subnetwork.region}")
///             .subnetwork("${subnetwork.id}")
///             .build_struct(),
///     );
///     let addrIntfRedundant = address::create(
///         "addrIntfRedundant",
///         AddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("my-router-addr-intf-red")
///             .region("${subnetwork.region}")
///             .subnetwork("${subnetwork.id}")
///             .build_struct(),
///     );
///     let addrPeer = address::create(
///         "addrPeer",
///         AddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("my-router-addr-peer")
///             .region("${subnetwork.region}")
///             .subnetwork("${subnetwork.id}")
///             .build_struct(),
///     );
///     let hub = hub::create(
///         "hub",
///         HubArgs::builder().name("my-router-hub").build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .boot_disk(
///                 InstanceBootDisk::builder()
///                     .initializeParams(
///                         InstanceBootDiskInitializeParams::builder()
///                             .image("debian-cloud/debian-11")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .can_ip_forward(true)
///             .machine_type("e2-medium")
///             .name("router-appliance")
///             .network_interfaces(
///                 vec![
///                     InstanceNetworkInterface::builder().networkIp("${addrPeer.address}")
///                     .subnetwork("${subnetwork.selfLink}").build_struct(),
///                 ],
///             )
///             .zone("us-central1-a")
///             .build_struct(),
///     );
///     let interface = router_interface::create(
///         "interface",
///         RouterInterfaceArgs::builder()
///             .name("my-router-intf")
///             .private_ip_address("${addrIntf.address}")
///             .redundant_interface("${interfaceRedundant.name}")
///             .region("${router.region}")
///             .router("${router.name}")
///             .subnetwork("${subnetwork.selfLink}")
///             .build_struct(),
///     );
///     let interfaceRedundant = router_interface::create(
///         "interfaceRedundant",
///         RouterInterfaceArgs::builder()
///             .name("my-router-intf-red")
///             .private_ip_address("${addrIntfRedundant.address}")
///             .region("${router.region}")
///             .router("${router.name}")
///             .subnetwork("${subnetwork.selfLink}")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-router-net")
///             .build_struct(),
///     );
///     let peer = router_peer::create(
///         "peer",
///         RouterPeerArgs::builder()
///             .interface("${interface.name}")
///             .name("my-router-peer")
///             .peer_asn(65513)
///             .peer_ip_address("${addrPeer.address}")
///             .region("${router.region}")
///             .router("${router.name}")
///             .router_appliance_instance("${instance.selfLink}")
///             .build_struct(),
///     );
///     let router = router::create(
///         "router",
///         RouterArgs::builder()
///             .bgp(RouterBgp::builder().asn(64514).build_struct())
///             .name("my-router-router")
///             .network("${network.selfLink}")
///             .region("${subnetwork.region}")
///             .build_struct(),
///     );
///     let spoke = spoke::create(
///         "spoke",
///         SpokeArgs::builder()
///             .hub("${hub.id}")
///             .linked_router_appliance_instances(
///                 SpokeLinkedRouterApplianceInstances::builder()
///                     .instances(
///                         vec![
///                             SpokeLinkedRouterApplianceInstancesInstance::builder()
///                             .ipAddress("${addrPeer.address}")
///                             .virtualMachine("${instance.selfLink}").build_struct(),
///                         ],
///                     )
///                     .siteToSiteDataTransfer(false)
///                     .build_struct(),
///             )
///             .location("${subnetwork.region}")
///             .name("my-router-spoke")
///             .build_struct(),
///     );
///     let subnetwork = subnetwork::create(
///         "subnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("my-router-sub")
///             .network("${network.selfLink}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Router Peer Md5 Authentication Key
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = router_peer::create(
///         "foobar",
///         RouterPeerArgs::builder()
///             .advertised_route_priority(100)
///             .interface("${foobarGoogleComputeRouterInterface.name}")
///             .md_5_authentication_key(
///                 RouterPeerMd5AuthenticationKey::builder()
///                     .key("%s-peer-key-value")
///                     .name("%s-peer-key")
///                     .build_struct(),
///             )
///             .name("%s-peer")
///             .peer_asn(65515)
///             .peer_ip_address("169.254.3.2")
///             .region("${foobarGoogleComputeRouter.region}")
///             .router("${foobarGoogleComputeRouter.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Router Peer Export And Import Policies
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: my-router-net
///       autoCreateSubnetworks: false
///   subnetwork:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-router-subnet
///       network: ${network.selfLink}
///       ipCidrRange: 10.0.0.0/16
///       region: us-central1
///   address:
///     type: gcp:compute:Address
///     properties:
///       name: my-router
///       region: ${subnetwork.region}
///   vpnGateway:
///     type: gcp:compute:HaVpnGateway
///     name: vpn_gateway
///     properties:
///       name: my-router-gateway
///       network: ${network.selfLink}
///       region: ${subnetwork.region}
///   externalGateway:
///     type: gcp:compute:ExternalVpnGateway
///     name: external_gateway
///     properties:
///       name: my-router-external-gateway
///       redundancyType: SINGLE_IP_INTERNALLY_REDUNDANT
///       description: An externally managed VPN gateway
///       interfaces:
///         - id: 0
///           ipAddress: 8.8.8.8
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: my-router
///       region: ${subnetwork.region}
///       network: ${network.selfLink}
///       bgp:
///         asn: 64514
///   vpnTunnel:
///     type: gcp:compute:VPNTunnel
///     name: vpn_tunnel
///     properties:
///       name: my-router
///       region: ${subnetwork.region}
///       vpnGateway: ${vpnGateway.id}
///       peerExternalGateway: ${externalGateway.id}
///       peerExternalGatewayInterface: 0
///       sharedSecret: unguessable
///       router: ${router.name}
///       vpnGatewayInterface: 0
///   routerInterface:
///     type: gcp:compute:RouterInterface
///     name: router_interface
///     properties:
///       name: my-router
///       router: ${router.name}
///       region: ${router.region}
///       vpnTunnel: ${vpnTunnel.name}
///   rp-export:
///     type: gcp:compute:RouterRoutePolicy
///     properties:
///       name: my-router-rp-export
///       router: ${router.name}
///       region: ${router.region}
///       type: ROUTE_POLICY_TYPE_EXPORT
///       terms:
///         - priority: 2
///           match:
///             expression: destination == '10.0.0.0/12'
///             title: export_expression
///             description: acceptance expression for export
///           actions:
///             - expression: accept()
///     options:
///       dependsOn:
///         - ${routerInterface}
///   rp-import:
///     type: gcp:compute:RouterRoutePolicy
///     properties:
///       name: my-router-rp-import
///       router: ${router.name}
///       region: ${router.region}
///       type: ROUTE_POLICY_TYPE_IMPORT
///       terms:
///         - priority: 1
///           match:
///             expression: destination == '10.0.0.0/12'
///             title: import_expression
///             description: acceptance expression for import
///           actions:
///             - expression: accept()
///     options:
///       dependsOn:
///         - ${routerInterface}
///         - ${["rp-export"]}
///   routerPeer:
///     type: gcp:compute:RouterPeer
///     name: router_peer
///     properties:
///       name: my-router-peer
///       router: ${router.name}
///       region: ${router.region}
///       peerAsn: 65515
///       advertisedRoutePriority: 100
///       interface: ${routerInterface.name}
///       md5AuthenticationKey:
///         name: my-router-peer-key
///         key: my-router-peer-key-value
///       importPolicies:
///         - ${["rp-import"].name}
///       exportPolicies:
///         - ${["rp-export"].name}
///     options:
///       dependsOn:
///         - ${["rp-export"]}
///         - ${["rp-import"]}
///         - ${routerInterface}
/// ```
///
/// ## Import
///
/// RouterBgpPeer can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/routers/{{router}}/{{name}}`
///
/// * `{{project}}/{{region}}/{{router}}/{{name}}`
///
/// * `{{region}}/{{router}}/{{name}}`
///
/// * `{{router}}/{{name}}`
///
/// When using the `pulumi import` command, RouterBgpPeer can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/routerPeer:RouterPeer default projects/{{project}}/regions/{{region}}/routers/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerPeer:RouterPeer default {{project}}/{{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerPeer:RouterPeer default {{region}}/{{router}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/routerPeer:RouterPeer default {{router}}/{{name}}
/// ```
///
pub mod router_peer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterPeerArgs {
        /// User-specified flag to indicate which mode to use for advertisement.
        /// Valid values of this enum field are: `DEFAULT`, `CUSTOM`
        /// Default value is `DEFAULT`.
        /// Possible values are: `DEFAULT`, `CUSTOM`.
        #[builder(into, default)]
        pub advertise_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User-specified list of prefix groups to advertise in custom
        /// mode, which currently supports the following option:
        /// * `ALL_SUBNETS`: Advertises all of the router's own VPC subnets.
        /// This excludes any routes learned for subnets that use VPC Network
        /// Peering.
        ///
        /// Note that this field can only be populated if advertiseMode is `CUSTOM`
        /// and overrides the list defined for the router (in the "bgp" message).
        /// These groups are advertised in addition to any specified prefixes.
        /// Leave this field blank to advertise no custom groups.
        #[builder(into, default)]
        pub advertised_groups: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// User-specified list of individual IP ranges to advertise in
        /// custom mode. This field can only be populated if advertiseMode
        /// is `CUSTOM` and is advertised to all peers of the router. These IP
        /// ranges will be advertised in addition to any specified groups.
        /// Leave this field blank to advertise no custom IP ranges.
        /// Structure is documented below.
        #[builder(into, default)]
        pub advertised_ip_ranges: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RouterPeerAdvertisedIpRange>>,
        >,
        /// The priority of routes advertised to this BGP peer.
        /// Where there is more than one matching route of maximum
        /// length, the routes with the lowest priority value win.
        #[builder(into, default)]
        pub advertised_route_priority: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// BFD configuration for the BGP peering.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bfd: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RouterPeerBfd>,
        >,
        /// The custom learned route IP address range. Must be a valid CIDR-formatted prefix.
        /// If an IP address is provided without a subnet mask, it is interpreted as, for IPv4,
        /// a /32 singular IP address range, and, for IPv6, /128.
        /// Structure is documented below.
        #[builder(into, default)]
        pub custom_learned_ip_ranges: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RouterPeerCustomLearnedIpRange>>,
        >,
        /// The user-defined custom learned route priority for a BGP session.
        /// This value is applied to all custom learned route ranges for the session.
        /// You can choose a value from 0 to 65335. If you don't provide a value,
        /// Google Cloud assigns a priority of 100 to the ranges.
        #[builder(into, default)]
        pub custom_learned_route_priority: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The status of the BGP peer connection. If set to false, any active session
        /// with the peer is terminated and all associated routing information is removed.
        /// If set to true, the peer connection can be established with routing information.
        /// The default is true.
        #[builder(into, default)]
        pub enable: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enable IPv4 traffic over BGP Peer. It is enabled by default if the peerIpAddress is version 4.
        #[builder(into, default)]
        pub enable_ipv4: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Enable IPv6 traffic over BGP Peer. If not specified, it is disabled by default.
        #[builder(into, default)]
        pub enable_ipv6: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// routers.list of export policies applied to this peer, in the order they must be evaluated.
        /// The name must correspond to an existing policy that has ROUTE_POLICY_TYPE_EXPORT type.
        #[builder(into, default)]
        pub export_policies: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// routers.list of import policies applied to this peer, in the order they must be evaluated.
        /// The name must correspond to an existing policy that has ROUTE_POLICY_TYPE_IMPORT type.
        #[builder(into, default)]
        pub import_policies: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the interface the BGP peer is associated with.
        #[builder(into)]
        pub interface: pulumi_wasm_rust::InputOrOutput<String>,
        /// IP address of the interface inside Google Cloud Platform.
        /// Only IPv4 is supported.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IPv4 address of the interface inside Google Cloud Platform.
        #[builder(into, default)]
        pub ipv4_nexthop_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IPv6 address of the interface inside Google Cloud Platform.
        /// The address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.
        /// If you do not specify the next hop addresses, Google Cloud automatically
        /// assigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you.
        #[builder(into, default)]
        pub ipv6_nexthop_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration for MD5 authentication on the BGP session.
        /// Structure is documented below.
        #[builder(into, default)]
        pub md5_authentication_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RouterPeerMd5AuthenticationKey>,
        >,
        /// Name of this BGP peer. The name must be 1-63 characters long,
        /// and comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which
        /// means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Peer BGP Autonomous System Number (ASN).
        /// Each BGP interface may use a different value.
        #[builder(into)]
        pub peer_asn: pulumi_wasm_rust::InputOrOutput<i32>,
        /// IP address of the BGP interface outside Google Cloud Platform.
        /// Only IPv4 is supported. Required if `ip_address` is set.
        #[builder(into, default)]
        pub peer_ip_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IPv4 address of the BGP interface outside Google Cloud Platform.
        #[builder(into, default)]
        pub peer_ipv4_nexthop_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IPv6 address of the BGP interface outside Google Cloud Platform.
        /// The address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.
        /// If you do not specify the next hop addresses, Google Cloud automatically
        /// assigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you.
        #[builder(into, default)]
        pub peer_ipv6_nexthop_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region where the router and BgpPeer reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Router in which this BgpPeer will be configured.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub router: pulumi_wasm_rust::InputOrOutput<String>,
        /// The URI of the VM instance that is used as third-party router appliances
        /// such as Next Gen Firewalls, Virtual Routers, or Router Appliances.
        /// The VM instance must be located in zones contained in the same region as
        /// this Cloud Router. The VM instance is the peer side of the BGP session.
        #[builder(into, default)]
        pub router_appliance_instance: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouterPeerResult {
        /// User-specified flag to indicate which mode to use for advertisement.
        /// Valid values of this enum field are: `DEFAULT`, `CUSTOM`
        /// Default value is `DEFAULT`.
        /// Possible values are: `DEFAULT`, `CUSTOM`.
        pub advertise_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// User-specified list of prefix groups to advertise in custom
        /// mode, which currently supports the following option:
        /// * `ALL_SUBNETS`: Advertises all of the router's own VPC subnets.
        /// This excludes any routes learned for subnets that use VPC Network
        /// Peering.
        ///
        /// Note that this field can only be populated if advertiseMode is `CUSTOM`
        /// and overrides the list defined for the router (in the "bgp" message).
        /// These groups are advertised in addition to any specified prefixes.
        /// Leave this field blank to advertise no custom groups.
        pub advertised_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// User-specified list of individual IP ranges to advertise in
        /// custom mode. This field can only be populated if advertiseMode
        /// is `CUSTOM` and is advertised to all peers of the router. These IP
        /// ranges will be advertised in addition to any specified groups.
        /// Leave this field blank to advertise no custom IP ranges.
        /// Structure is documented below.
        pub advertised_ip_ranges: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RouterPeerAdvertisedIpRange>>,
        >,
        /// The priority of routes advertised to this BGP peer.
        /// Where there is more than one matching route of maximum
        /// length, the routes with the lowest priority value win.
        pub advertised_route_priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// BFD configuration for the BGP peering.
        /// Structure is documented below.
        pub bfd: pulumi_wasm_rust::Output<super::super::types::compute::RouterPeerBfd>,
        /// The custom learned route IP address range. Must be a valid CIDR-formatted prefix.
        /// If an IP address is provided without a subnet mask, it is interpreted as, for IPv4,
        /// a /32 singular IP address range, and, for IPv6, /128.
        /// Structure is documented below.
        pub custom_learned_ip_ranges: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RouterPeerCustomLearnedIpRange>>,
        >,
        /// The user-defined custom learned route priority for a BGP session.
        /// This value is applied to all custom learned route ranges for the session.
        /// You can choose a value from 0 to 65335. If you don't provide a value,
        /// Google Cloud assigns a priority of 100 to the ranges.
        pub custom_learned_route_priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The status of the BGP peer connection. If set to false, any active session
        /// with the peer is terminated and all associated routing information is removed.
        /// If set to true, the peer connection can be established with routing information.
        /// The default is true.
        pub enable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable IPv4 traffic over BGP Peer. It is enabled by default if the peerIpAddress is version 4.
        pub enable_ipv4: pulumi_wasm_rust::Output<bool>,
        /// Enable IPv6 traffic over BGP Peer. If not specified, it is disabled by default.
        pub enable_ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// routers.list of export policies applied to this peer, in the order they must be evaluated.
        /// The name must correspond to an existing policy that has ROUTE_POLICY_TYPE_EXPORT type.
        pub export_policies: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// routers.list of import policies applied to this peer, in the order they must be evaluated.
        /// The name must correspond to an existing policy that has ROUTE_POLICY_TYPE_IMPORT type.
        pub import_policies: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the interface the BGP peer is associated with.
        pub interface: pulumi_wasm_rust::Output<String>,
        /// IP address of the interface inside Google Cloud Platform.
        /// Only IPv4 is supported.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// IPv4 address of the interface inside Google Cloud Platform.
        pub ipv4_nexthop_address: pulumi_wasm_rust::Output<String>,
        /// IPv6 address of the interface inside Google Cloud Platform.
        /// The address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.
        /// If you do not specify the next hop addresses, Google Cloud automatically
        /// assigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you.
        pub ipv6_nexthop_address: pulumi_wasm_rust::Output<String>,
        /// The resource that configures and manages this BGP peer.
        /// * `MANAGED_BY_USER` is the default value and can be managed by
        /// you or other users
        /// * `MANAGED_BY_ATTACHMENT` is a BGP peer that is configured and
        /// managed by Cloud Interconnect, specifically by an
        /// InterconnectAttachment of type PARTNER. Google automatically
        /// creates, updates, and deletes this type of BGP peer when the
        /// PARTNER InterconnectAttachment is created, updated,
        /// or deleted.
        pub management_type: pulumi_wasm_rust::Output<String>,
        /// Configuration for MD5 authentication on the BGP session.
        /// Structure is documented below.
        pub md5_authentication_key: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RouterPeerMd5AuthenticationKey>,
        >,
        /// Name of this BGP peer. The name must be 1-63 characters long,
        /// and comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?` which
        /// means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Peer BGP Autonomous System Number (ASN).
        /// Each BGP interface may use a different value.
        pub peer_asn: pulumi_wasm_rust::Output<i32>,
        /// IP address of the BGP interface outside Google Cloud Platform.
        /// Only IPv4 is supported. Required if `ip_address` is set.
        pub peer_ip_address: pulumi_wasm_rust::Output<String>,
        /// IPv4 address of the BGP interface outside Google Cloud Platform.
        pub peer_ipv4_nexthop_address: pulumi_wasm_rust::Output<String>,
        /// IPv6 address of the BGP interface outside Google Cloud Platform.
        /// The address must be in the range 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64.
        /// If you do not specify the next hop addresses, Google Cloud automatically
        /// assigns unused addresses from the 2600:2d00:0:2::/64 or 2600:2d00:0:3::/64 range for you.
        pub peer_ipv6_nexthop_address: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where the router and BgpPeer reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The name of the Cloud Router in which this BgpPeer will be configured.
        ///
        ///
        /// - - -
        pub router: pulumi_wasm_rust::Output<String>,
        /// The URI of the VM instance that is used as third-party router appliances
        /// such as Next Gen Firewalls, Virtual Routers, or Router Appliances.
        /// The VM instance must be located in zones contained in the same region as
        /// this Cloud Router. The VM instance is the peer side of the BGP session.
        pub router_appliance_instance: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RouterPeerArgs,
    ) -> RouterPeerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advertise_mode_binding = args.advertise_mode.get_output(context).get_inner();
        let advertised_groups_binding = args
            .advertised_groups
            .get_output(context)
            .get_inner();
        let advertised_ip_ranges_binding = args
            .advertised_ip_ranges
            .get_output(context)
            .get_inner();
        let advertised_route_priority_binding = args
            .advertised_route_priority
            .get_output(context)
            .get_inner();
        let bfd_binding = args.bfd.get_output(context).get_inner();
        let custom_learned_ip_ranges_binding = args
            .custom_learned_ip_ranges
            .get_output(context)
            .get_inner();
        let custom_learned_route_priority_binding = args
            .custom_learned_route_priority
            .get_output(context)
            .get_inner();
        let enable_binding = args.enable.get_output(context).get_inner();
        let enable_ipv4_binding = args.enable_ipv4.get_output(context).get_inner();
        let enable_ipv6_binding = args.enable_ipv6.get_output(context).get_inner();
        let export_policies_binding = args
            .export_policies
            .get_output(context)
            .get_inner();
        let import_policies_binding = args
            .import_policies
            .get_output(context)
            .get_inner();
        let interface_binding = args.interface.get_output(context).get_inner();
        let ip_address_binding = args.ip_address.get_output(context).get_inner();
        let ipv4_nexthop_address_binding = args
            .ipv4_nexthop_address
            .get_output(context)
            .get_inner();
        let ipv6_nexthop_address_binding = args
            .ipv6_nexthop_address
            .get_output(context)
            .get_inner();
        let md5_authentication_key_binding = args
            .md5_authentication_key
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let peer_asn_binding = args.peer_asn.get_output(context).get_inner();
        let peer_ip_address_binding = args
            .peer_ip_address
            .get_output(context)
            .get_inner();
        let peer_ipv4_nexthop_address_binding = args
            .peer_ipv4_nexthop_address
            .get_output(context)
            .get_inner();
        let peer_ipv6_nexthop_address_binding = args
            .peer_ipv6_nexthop_address
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let router_binding = args.router.get_output(context).get_inner();
        let router_appliance_instance_binding = args
            .router_appliance_instance
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/routerPeer:RouterPeer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advertiseMode".into(),
                    value: &advertise_mode_binding,
                },
                register_interface::ObjectField {
                    name: "advertisedGroups".into(),
                    value: &advertised_groups_binding,
                },
                register_interface::ObjectField {
                    name: "advertisedIpRanges".into(),
                    value: &advertised_ip_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "advertisedRoutePriority".into(),
                    value: &advertised_route_priority_binding,
                },
                register_interface::ObjectField {
                    name: "bfd".into(),
                    value: &bfd_binding,
                },
                register_interface::ObjectField {
                    name: "customLearnedIpRanges".into(),
                    value: &custom_learned_ip_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "customLearnedRoutePriority".into(),
                    value: &custom_learned_route_priority_binding,
                },
                register_interface::ObjectField {
                    name: "enable".into(),
                    value: &enable_binding,
                },
                register_interface::ObjectField {
                    name: "enableIpv4".into(),
                    value: &enable_ipv4_binding,
                },
                register_interface::ObjectField {
                    name: "enableIpv6".into(),
                    value: &enable_ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "exportPolicies".into(),
                    value: &export_policies_binding,
                },
                register_interface::ObjectField {
                    name: "importPolicies".into(),
                    value: &import_policies_binding,
                },
                register_interface::ObjectField {
                    name: "interface".into(),
                    value: &interface_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4NexthopAddress".into(),
                    value: &ipv4_nexthop_address_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6NexthopAddress".into(),
                    value: &ipv6_nexthop_address_binding,
                },
                register_interface::ObjectField {
                    name: "md5AuthenticationKey".into(),
                    value: &md5_authentication_key_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "peerAsn".into(),
                    value: &peer_asn_binding,
                },
                register_interface::ObjectField {
                    name: "peerIpAddress".into(),
                    value: &peer_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "peerIpv4NexthopAddress".into(),
                    value: &peer_ipv4_nexthop_address_binding,
                },
                register_interface::ObjectField {
                    name: "peerIpv6NexthopAddress".into(),
                    value: &peer_ipv6_nexthop_address_binding,
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
                    name: "router".into(),
                    value: &router_binding,
                },
                register_interface::ObjectField {
                    name: "routerApplianceInstance".into(),
                    value: &router_appliance_instance_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "advertiseMode".into(),
                },
                register_interface::ResultField {
                    name: "advertisedGroups".into(),
                },
                register_interface::ResultField {
                    name: "advertisedIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "advertisedRoutePriority".into(),
                },
                register_interface::ResultField {
                    name: "bfd".into(),
                },
                register_interface::ResultField {
                    name: "customLearnedIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "customLearnedRoutePriority".into(),
                },
                register_interface::ResultField {
                    name: "enable".into(),
                },
                register_interface::ResultField {
                    name: "enableIpv4".into(),
                },
                register_interface::ResultField {
                    name: "enableIpv6".into(),
                },
                register_interface::ResultField {
                    name: "exportPolicies".into(),
                },
                register_interface::ResultField {
                    name: "importPolicies".into(),
                },
                register_interface::ResultField {
                    name: "interface".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "ipv4NexthopAddress".into(),
                },
                register_interface::ResultField {
                    name: "ipv6NexthopAddress".into(),
                },
                register_interface::ResultField {
                    name: "managementType".into(),
                },
                register_interface::ResultField {
                    name: "md5AuthenticationKey".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peerAsn".into(),
                },
                register_interface::ResultField {
                    name: "peerIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "peerIpv4NexthopAddress".into(),
                },
                register_interface::ResultField {
                    name: "peerIpv6NexthopAddress".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "router".into(),
                },
                register_interface::ResultField {
                    name: "routerApplianceInstance".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouterPeerResult {
            advertise_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advertiseMode").unwrap(),
            ),
            advertised_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advertisedGroups").unwrap(),
            ),
            advertised_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advertisedIpRanges").unwrap(),
            ),
            advertised_route_priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advertisedRoutePriority").unwrap(),
            ),
            bfd: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bfd").unwrap(),
            ),
            custom_learned_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLearnedIpRanges").unwrap(),
            ),
            custom_learned_route_priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLearnedRoutePriority").unwrap(),
            ),
            enable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enable").unwrap(),
            ),
            enable_ipv4: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableIpv4").unwrap(),
            ),
            enable_ipv6: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableIpv6").unwrap(),
            ),
            export_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportPolicies").unwrap(),
            ),
            import_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importPolicies").unwrap(),
            ),
            interface: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interface").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            ipv4_nexthop_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4NexthopAddress").unwrap(),
            ),
            ipv6_nexthop_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6NexthopAddress").unwrap(),
            ),
            management_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementType").unwrap(),
            ),
            md5_authentication_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("md5AuthenticationKey").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peer_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAsn").unwrap(),
            ),
            peer_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerIpAddress").unwrap(),
            ),
            peer_ipv4_nexthop_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerIpv4NexthopAddress").unwrap(),
            ),
            peer_ipv6_nexthop_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerIpv6NexthopAddress").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            router: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("router").unwrap(),
            ),
            router_appliance_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routerApplianceInstance").unwrap(),
            ),
        }
    }
}
