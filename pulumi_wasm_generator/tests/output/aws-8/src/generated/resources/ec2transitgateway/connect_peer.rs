/// Manages an EC2 Transit Gateway Connect Peer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connect::create(
///         "example",
///         ConnectArgs::builder()
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .transport_attachment_id("${exampleAwsEc2TransitGatewayVpcAttachment.id}")
///             .build_struct(),
///     );
///     let exampleConnectPeer = connect_peer::create(
///         "exampleConnectPeer",
///         ConnectPeerArgs::builder()
///             .inside_cidr_blocks(vec!["169.254.100.0/29",])
///             .peer_address("10.1.2.3")
///             .transit_gateway_attachment_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_connect_peer` using the EC2 Transit Gateway Connect Peer identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/connectPeer:ConnectPeer example tgw-connect-peer-12345678
/// ```
pub mod connect_peer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectPeerArgs {
        /// The BGP ASN number assigned customer device. If not provided, it will use the same BGP ASN as is associated with Transit Gateway.
        #[builder(into, default)]
        pub bgp_asn: pulumi_wasm_rust::Output<Option<String>>,
        /// The CIDR block that will be used for addressing within the tunnel. It must contain exactly one IPv4 CIDR block and up to one IPv6 CIDR block. The IPv4 CIDR block must be /29 size and must be within 169.254.0.0/16 range, with exception of: 169.254.0.0/29, 169.254.1.0/29, 169.254.2.0/29, 169.254.3.0/29, 169.254.4.0/29, 169.254.5.0/29, 169.254.169.248/29. The IPv6 CIDR block must be /125 size and must be within fd00::/8. The first IP from each CIDR block is assigned for customer gateway, the second and third is for Transit Gateway (An example: from range 169.254.100.0/29, .1 is assigned to customer gateway and .2 and .3 are assigned to Transit Gateway)
        #[builder(into)]
        pub inside_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The IP addressed assigned to customer device, which will be used as tunnel endpoint. It can be IPv4 or IPv6 address, but must be the same address family as `transit_gateway_address`
        #[builder(into)]
        pub peer_address: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Connect Peer. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IP address assigned to Transit Gateway, which will be used as tunnel endpoint. This address must be from associated Transit Gateway CIDR block. The address must be from the same address family as `peer_address`. If not set explicitly, it will be selected from associated Transit Gateway CIDR blocks
        #[builder(into, default)]
        pub transit_gateway_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The Transit Gateway Connect
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectPeerResult {
        /// EC2 Transit Gateway Connect Peer ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The BGP ASN number assigned customer device. If not provided, it will use the same BGP ASN as is associated with Transit Gateway.
        pub bgp_asn: pulumi_wasm_rust::Output<String>,
        /// The IP address assigned to customer device, which is used as BGP IP address.
        pub bgp_peer_address: pulumi_wasm_rust::Output<String>,
        /// The IP addresses assigned to Transit Gateway, which are used as BGP IP addresses.
        pub bgp_transit_gateway_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The CIDR block that will be used for addressing within the tunnel. It must contain exactly one IPv4 CIDR block and up to one IPv6 CIDR block. The IPv4 CIDR block must be /29 size and must be within 169.254.0.0/16 range, with exception of: 169.254.0.0/29, 169.254.1.0/29, 169.254.2.0/29, 169.254.3.0/29, 169.254.4.0/29, 169.254.5.0/29, 169.254.169.248/29. The IPv6 CIDR block must be /125 size and must be within fd00::/8. The first IP from each CIDR block is assigned for customer gateway, the second and third is for Transit Gateway (An example: from range 169.254.100.0/29, .1 is assigned to customer gateway and .2 and .3 are assigned to Transit Gateway)
        pub inside_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The IP addressed assigned to customer device, which will be used as tunnel endpoint. It can be IPv4 or IPv6 address, but must be the same address family as `transit_gateway_address`
        pub peer_address: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Connect Peer. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP address assigned to Transit Gateway, which will be used as tunnel endpoint. This address must be from associated Transit Gateway CIDR block. The address must be from the same address family as `peer_address`. If not set explicitly, it will be selected from associated Transit Gateway CIDR blocks
        pub transit_gateway_address: pulumi_wasm_rust::Output<String>,
        /// The Transit Gateway Connect
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectPeerArgs) -> ConnectPeerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bgp_asn_binding = args.bgp_asn.get_inner();
        let inside_cidr_blocks_binding = args.inside_cidr_blocks.get_inner();
        let peer_address_binding = args.peer_address.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_address_binding = args.transit_gateway_address.get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/connectPeer:ConnectPeer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bgpAsn".into(),
                    value: &bgp_asn_binding,
                },
                register_interface::ObjectField {
                    name: "insideCidrBlocks".into(),
                    value: &inside_cidr_blocks_binding,
                },
                register_interface::ObjectField {
                    name: "peerAddress".into(),
                    value: &peer_address_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAddress".into(),
                    value: &transit_gateway_address_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "bgpPeerAddress".into(),
                },
                register_interface::ResultField {
                    name: "bgpTransitGatewayAddresses".into(),
                },
                register_interface::ResultField {
                    name: "insideCidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "peerAddress".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAddress".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectPeerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpAsn").unwrap(),
            ),
            bgp_peer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpPeerAddress").unwrap(),
            ),
            bgp_transit_gateway_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpTransitGatewayAddresses").unwrap(),
            ),
            inside_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insideCidrBlocks").unwrap(),
            ),
            peer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAddress").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAddress").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
        }
    }
}
