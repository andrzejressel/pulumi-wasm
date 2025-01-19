/// Represents an InterconnectAttachment (VLAN attachment) resource. For more
/// information, see Creating VLAN Attachments.
///
///
///
/// ## Example Usage
///
/// ### Interconnect Attachment Basic
///
///
/// ```yaml
/// resources:
///   onPrem:
///     type: gcp:compute:InterconnectAttachment
///     name: on_prem
///     properties:
///       name: on-prem-attachment
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_1
///       type: PARTNER
///       router: ${foobar.id}
///       mtu: 1500
///   foobar:
///     type: gcp:compute:Router
///     properties:
///       name: router-1
///       network: ${foobarNetwork.name}
///       bgp:
///         asn: 16550
///   foobarNetwork:
///     type: gcp:compute:Network
///     name: foobar
///     properties:
///       name: network-1
///       autoCreateSubnetworks: false
/// ```
/// ### Compute Interconnect Attachment Ipsec Encryption
///
///
/// ```yaml
/// resources:
///   ipsec-encrypted-interconnect-attachment:
///     type: gcp:compute:InterconnectAttachment
///     properties:
///       name: test-interconnect-attachment
///       edgeAvailabilityDomain: AVAILABILITY_DOMAIN_1
///       type: PARTNER
///       router: ${router.id}
///       encryption: IPSEC
///       ipsecInternalAddresses:
///         - ${address.selfLink}
///   address:
///     type: gcp:compute:Address
///     properties:
///       name: test-address
///       addressType: INTERNAL
///       purpose: IPSEC_INTERCONNECT
///       address: 192.168.1.0
///       prefixLength: 29
///       network: ${network.selfLink}
///   router:
///     type: gcp:compute:Router
///     properties:
///       name: test-router
///       network: ${network.name}
///       encryptedInterconnectRouter: true
///       bgp:
///         asn: 16550
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: test-network
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// InterconnectAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/interconnectAttachments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, InterconnectAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/interconnectAttachment:InterconnectAttachment default projects/{{project}}/regions/{{region}}/interconnectAttachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/interconnectAttachment:InterconnectAttachment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/interconnectAttachment:InterconnectAttachment default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/interconnectAttachment:InterconnectAttachment default {{name}}
/// ```
///
pub mod interconnect_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InterconnectAttachmentArgs {
        /// Whether the VLAN attachment is enabled or disabled.  When using
        /// PARTNER type this will Pre-Activate the interconnect attachment
        #[builder(into, default)]
        pub admin_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Provisioned bandwidth capacity for the interconnect attachment.
        /// For attachments of type DEDICATED, the user can set the bandwidth.
        /// For attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.
        /// Output only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,
        /// Defaults to BPS_10G
        /// Possible values are: `BPS_50M`, `BPS_100M`, `BPS_200M`, `BPS_300M`, `BPS_400M`, `BPS_500M`, `BPS_1G`, `BPS_2G`, `BPS_5G`, `BPS_10G`, `BPS_20G`, `BPS_50G`.
        #[builder(into, default)]
        pub bandwidth: pulumi_wasm_rust::Output<Option<String>>,
        /// Up to 16 candidate prefixes that can be used to restrict the allocation
        /// of cloudRouterIpAddress and customerRouterIpAddress for this attachment.
        /// All prefixes must be within link-local address space (169.254.0.0/16)
        /// and must be /29 or shorter (/28, /27, etc). Google will attempt to select
        /// an unused /29 from the supplied candidate prefix(es). The request will
        /// fail if all possible /29s are in use on Google's edge. If not supplied,
        /// Google will randomly select an unused /29 from all of link-local space.
        #[builder(into, default)]
        pub candidate_subnets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Desired availability domain for the attachment. Only available for type
        /// PARTNER, at creation time. For improved reliability, customers should
        /// configure a pair of attachments with one per availability domain. The
        /// selected availability domain will be provided to the Partner via the
        /// pairing key so that the provisioned circuit will lie in the specified
        /// domain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY.
        #[builder(into, default)]
        pub edge_availability_domain: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the user-supplied encryption option of this interconnect
        /// attachment. Can only be specified at attachment creation for PARTNER or
        /// DEDICATED attachments.
        /// * NONE - This is the default value, which means that the VLAN attachment
        /// carries unencrypted traffic. VMs are able to send traffic to, or receive
        /// traffic from, such a VLAN attachment.
        /// * IPSEC - The VLAN attachment carries only encrypted traffic that is
        /// encrypted by an IPsec device, such as an HA VPN gateway or third-party
        /// IPsec VPN. VMs cannot directly send traffic to, or receive traffic from,
        /// such a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN
        /// attachment must be created with this option.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `IPSEC`.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the underlying Interconnect object that this attachment's
        /// traffic will traverse through. Required if type is DEDICATED, must not
        /// be set if type is PARTNER.
        #[builder(into, default)]
        pub interconnect: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of addresses that have been reserved for the interconnect attachment,
        /// Used only for interconnect attachment that has the encryption option as
        /// IPSEC.
        /// The addresses must be RFC 1918 IP address ranges. When creating HA VPN
        /// gateway over the interconnect attachment, if the attachment is configured
        /// to use an RFC 1918 IP address, then the VPN gateway's IP address will be
        /// allocated from the IP address range specified here.
        /// For example, if the HA VPN gateway's interface 0 is paired to this
        /// interconnect attachment, then an RFC 1918 IP address for the VPN gateway
        /// interface 0 will be allocated from the IP address specified for this
        /// interconnect attachment.
        /// If this field is not specified for interconnect attachment that has
        /// encryption option as IPSEC, later on when creating HA VPN gateway on this
        /// interconnect attachment, the HA VPN gateway's IP address will be
        /// allocated from regional external IP address pool.
        #[builder(into, default)]
        pub ipsec_internal_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Maximum Transmission Unit (MTU), in bytes, of packets passing through
        /// this interconnect attachment. Currently, only 1440 and 1500 are allowed. If not specified, the value will default to 1440.
        #[builder(into, default)]
        pub mtu: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is created. The
        /// name must be 1-63 characters long, and comply with RFC1035. Specifically, the
        /// name must be 1-63 characters long and match the regular expression
        /// `a-z?` which means the first character must be a
        /// lowercase letter, and all following characters must be a dash, lowercase
        /// letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Region where the regional interconnect attachment resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the cloud router to be used for dynamic routing. This router must be in
        /// the same region as this InterconnectAttachment. The InterconnectAttachment will
        /// automatically connect the Interconnect to the network & region within which the
        /// Cloud Router is configured.
        #[builder(into)]
        pub router: pulumi_wasm_rust::Output<String>,
        /// The stack type for this interconnect attachment to identify whether the IPv6
        /// feature is enabled or not. If not specified, IPV4_ONLY will be used.
        /// This field can be both set at interconnect attachments creation and update
        /// interconnect attachment operations.
        /// Possible values are: `IPV4_IPV6`, `IPV4_ONLY`.
        #[builder(into, default)]
        pub stack_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Length of the IPv4 subnet mask. Allowed values: 29 (default), 30. The default value is 29,
        /// except for Cross-Cloud Interconnect connections that use an InterconnectRemoteLocation with a
        /// constraints.subnetLengthRange.min equal to 30. For example, connections that use an Azure
        /// remote location fall into this category. In these cases, the default value is 30, and
        /// requesting 29 returns an error. Where both 29 and 30 are allowed, 29 is preferred, because it
        /// gives Google Cloud Support more debugging visibility.
        #[builder(into, default)]
        pub subnet_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The type of InterconnectAttachment you wish to create. Defaults to
        /// DEDICATED.
        /// Possible values are: `DEDICATED`, `PARTNER`, `PARTNER_PROVIDER`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When
        /// using PARTNER type this will be managed upstream.
        #[builder(into, default)]
        pub vlan_tag8021q: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct InterconnectAttachmentResult {
        /// Whether the VLAN attachment is enabled or disabled.  When using
        /// PARTNER type this will Pre-Activate the interconnect attachment
        pub admin_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Provisioned bandwidth capacity for the interconnect attachment.
        /// For attachments of type DEDICATED, the user can set the bandwidth.
        /// For attachments of type PARTNER, the Google Partner that is operating the interconnect must set the bandwidth.
        /// Output only for PARTNER type, mutable for PARTNER_PROVIDER and DEDICATED,
        /// Defaults to BPS_10G
        /// Possible values are: `BPS_50M`, `BPS_100M`, `BPS_200M`, `BPS_300M`, `BPS_400M`, `BPS_500M`, `BPS_1G`, `BPS_2G`, `BPS_5G`, `BPS_10G`, `BPS_20G`, `BPS_50G`.
        pub bandwidth: pulumi_wasm_rust::Output<String>,
        /// Up to 16 candidate prefixes that can be used to restrict the allocation
        /// of cloudRouterIpAddress and customerRouterIpAddress for this attachment.
        /// All prefixes must be within link-local address space (169.254.0.0/16)
        /// and must be /29 or shorter (/28, /27, etc). Google will attempt to select
        /// an unused /29 from the supplied candidate prefix(es). The request will
        /// fail if all possible /29s are in use on Google's edge. If not supplied,
        /// Google will randomly select an unused /29 from all of link-local space.
        pub candidate_subnets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// IPv4 address + prefix length to be configured on Cloud Router
        /// Interface for this interconnect attachment.
        pub cloud_router_ip_address: pulumi_wasm_rust::Output<String>,
        /// IPv6 address + prefix length to be configured on Cloud Router
        /// Interface for this interconnect attachment.
        pub cloud_router_ipv6_address: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// IPv4 address + prefix length to be configured on the customer
        /// router subinterface for this interconnect attachment.
        pub customer_router_ip_address: pulumi_wasm_rust::Output<String>,
        /// IPv6 address + prefix length to be configured on the customer
        /// router subinterface for this interconnect attachment.
        pub customer_router_ipv6_address: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Desired availability domain for the attachment. Only available for type
        /// PARTNER, at creation time. For improved reliability, customers should
        /// configure a pair of attachments with one per availability domain. The
        /// selected availability domain will be provided to the Partner via the
        /// pairing key so that the provisioned circuit will lie in the specified
        /// domain. If not specified, the value will default to AVAILABILITY_DOMAIN_ANY.
        pub edge_availability_domain: pulumi_wasm_rust::Output<String>,
        /// Indicates the user-supplied encryption option of this interconnect
        /// attachment. Can only be specified at attachment creation for PARTNER or
        /// DEDICATED attachments.
        /// * NONE - This is the default value, which means that the VLAN attachment
        /// carries unencrypted traffic. VMs are able to send traffic to, or receive
        /// traffic from, such a VLAN attachment.
        /// * IPSEC - The VLAN attachment carries only encrypted traffic that is
        /// encrypted by an IPsec device, such as an HA VPN gateway or third-party
        /// IPsec VPN. VMs cannot directly send traffic to, or receive traffic from,
        /// such a VLAN attachment. To use HA VPN over Cloud Interconnect, the VLAN
        /// attachment must be created with this option.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `IPSEC`.
        pub encryption: pulumi_wasm_rust::Output<Option<String>>,
        /// Google reference ID, to be used when raising support tickets with
        /// Google or otherwise to debug backend connectivity issues.
        pub google_reference_id: pulumi_wasm_rust::Output<String>,
        /// URL of the underlying Interconnect object that this attachment's
        /// traffic will traverse through. Required if type is DEDICATED, must not
        /// be set if type is PARTNER.
        pub interconnect: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of addresses that have been reserved for the interconnect attachment,
        /// Used only for interconnect attachment that has the encryption option as
        /// IPSEC.
        /// The addresses must be RFC 1918 IP address ranges. When creating HA VPN
        /// gateway over the interconnect attachment, if the attachment is configured
        /// to use an RFC 1918 IP address, then the VPN gateway's IP address will be
        /// allocated from the IP address range specified here.
        /// For example, if the HA VPN gateway's interface 0 is paired to this
        /// interconnect attachment, then an RFC 1918 IP address for the VPN gateway
        /// interface 0 will be allocated from the IP address specified for this
        /// interconnect attachment.
        /// If this field is not specified for interconnect attachment that has
        /// encryption option as IPSEC, later on when creating HA VPN gateway on this
        /// interconnect attachment, the HA VPN gateway's IP address will be
        /// allocated from regional external IP address pool.
        pub ipsec_internal_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Maximum Transmission Unit (MTU), in bytes, of packets passing through
        /// this interconnect attachment. Currently, only 1440 and 1500 are allowed. If not specified, the value will default to 1440.
        pub mtu: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is created. The
        /// name must be 1-63 characters long, and comply with RFC1035. Specifically, the
        /// name must be 1-63 characters long and match the regular expression
        /// `a-z?` which means the first character must be a
        /// lowercase letter, and all following characters must be a dash, lowercase
        /// letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// [Output only for type PARTNER. Not present for DEDICATED]. The opaque
        /// identifier of an PARTNER attachment used to initiate provisioning with
        /// a selected partner. Of the form "XXXXX/region/domain"
        pub pairing_key: pulumi_wasm_rust::Output<String>,
        /// [Output only for type PARTNER. Not present for DEDICATED]. Optional
        /// BGP ASN for the router that should be supplied by a layer 3 Partner if
        /// they configured BGP on behalf of the customer.
        pub partner_asn: pulumi_wasm_rust::Output<String>,
        /// Information specific to an InterconnectAttachment. This property
        /// is populated if the interconnect that this is attached to is of type DEDICATED.
        /// Structure is documented below.
        pub private_interconnect_infos: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::compute::InterconnectAttachmentPrivateInterconnectInfo,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where the regional interconnect attachment resides.
        pub region: pulumi_wasm_rust::Output<String>,
        /// URL of the cloud router to be used for dynamic routing. This router must be in
        /// the same region as this InterconnectAttachment. The InterconnectAttachment will
        /// automatically connect the Interconnect to the network & region within which the
        /// Cloud Router is configured.
        pub router: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The stack type for this interconnect attachment to identify whether the IPv6
        /// feature is enabled or not. If not specified, IPV4_ONLY will be used.
        /// This field can be both set at interconnect attachments creation and update
        /// interconnect attachment operations.
        /// Possible values are: `IPV4_IPV6`, `IPV4_ONLY`.
        pub stack_type: pulumi_wasm_rust::Output<String>,
        /// [Output Only] The current state of this attachment's functionality.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Length of the IPv4 subnet mask. Allowed values: 29 (default), 30. The default value is 29,
        /// except for Cross-Cloud Interconnect connections that use an InterconnectRemoteLocation with a
        /// constraints.subnetLengthRange.min equal to 30. For example, connections that use an Azure
        /// remote location fall into this category. In these cases, the default value is 30, and
        /// requesting 29 returns an error. Where both 29 and 30 are allowed, 29 is preferred, because it
        /// gives Google Cloud Support more debugging visibility.
        pub subnet_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The type of InterconnectAttachment you wish to create. Defaults to
        /// DEDICATED.
        /// Possible values are: `DEDICATED`, `PARTNER`, `PARTNER_PROVIDER`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The IEEE 802.1Q VLAN tag for this attachment, in the range 2-4094. When
        /// using PARTNER type this will be managed upstream.
        pub vlan_tag8021q: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InterconnectAttachmentArgs,
    ) -> InterconnectAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_enabled_binding = args.admin_enabled.get_inner();
        let bandwidth_binding = args.bandwidth.get_inner();
        let candidate_subnets_binding = args.candidate_subnets.get_inner();
        let description_binding = args.description.get_inner();
        let edge_availability_domain_binding = args.edge_availability_domain.get_inner();
        let encryption_binding = args.encryption.get_inner();
        let interconnect_binding = args.interconnect.get_inner();
        let ipsec_internal_addresses_binding = args.ipsec_internal_addresses.get_inner();
        let mtu_binding = args.mtu.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let router_binding = args.router.get_inner();
        let stack_type_binding = args.stack_type.get_inner();
        let subnet_length_binding = args.subnet_length.get_inner();
        let type__binding = args.type_.get_inner();
        let vlan_tag8021q_binding = args.vlan_tag8021q.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/interconnectAttachment:InterconnectAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminEnabled".into(),
                    value: &admin_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "bandwidth".into(),
                    value: &bandwidth_binding,
                },
                register_interface::ObjectField {
                    name: "candidateSubnets".into(),
                    value: &candidate_subnets_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "edgeAvailabilityDomain".into(),
                    value: &edge_availability_domain_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "interconnect".into(),
                    value: &interconnect_binding,
                },
                register_interface::ObjectField {
                    name: "ipsecInternalAddresses".into(),
                    value: &ipsec_internal_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "mtu".into(),
                    value: &mtu_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "stackType".into(),
                    value: &stack_type_binding,
                },
                register_interface::ObjectField {
                    name: "subnetLength".into(),
                    value: &subnet_length_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vlanTag8021q".into(),
                    value: &vlan_tag8021q_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminEnabled".into(),
                },
                register_interface::ResultField {
                    name: "bandwidth".into(),
                },
                register_interface::ResultField {
                    name: "candidateSubnets".into(),
                },
                register_interface::ResultField {
                    name: "cloudRouterIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "cloudRouterIpv6Address".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "customerRouterIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "customerRouterIpv6Address".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "edgeAvailabilityDomain".into(),
                },
                register_interface::ResultField {
                    name: "encryption".into(),
                },
                register_interface::ResultField {
                    name: "googleReferenceId".into(),
                },
                register_interface::ResultField {
                    name: "interconnect".into(),
                },
                register_interface::ResultField {
                    name: "ipsecInternalAddresses".into(),
                },
                register_interface::ResultField {
                    name: "mtu".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pairingKey".into(),
                },
                register_interface::ResultField {
                    name: "partnerAsn".into(),
                },
                register_interface::ResultField {
                    name: "privateInterconnectInfos".into(),
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
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "stackType".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetLength".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vlanTag8021q".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InterconnectAttachmentResult {
            admin_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminEnabled").unwrap(),
            ),
            bandwidth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bandwidth").unwrap(),
            ),
            candidate_subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("candidateSubnets").unwrap(),
            ),
            cloud_router_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudRouterIpAddress").unwrap(),
            ),
            cloud_router_ipv6_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudRouterIpv6Address").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            customer_router_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerRouterIpAddress").unwrap(),
            ),
            customer_router_ipv6_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerRouterIpv6Address").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            edge_availability_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeAvailabilityDomain").unwrap(),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryption").unwrap(),
            ),
            google_reference_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("googleReferenceId").unwrap(),
            ),
            interconnect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interconnect").unwrap(),
            ),
            ipsec_internal_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipsecInternalAddresses").unwrap(),
            ),
            mtu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mtu").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pairing_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pairingKey").unwrap(),
            ),
            partner_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerAsn").unwrap(),
            ),
            private_interconnect_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateInterconnectInfos").unwrap(),
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
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            stack_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackType").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetLength").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vlan_tag8021q: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlanTag8021q").unwrap(),
            ),
        }
    }
}
