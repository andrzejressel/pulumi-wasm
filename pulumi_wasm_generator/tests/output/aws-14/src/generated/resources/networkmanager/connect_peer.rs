/// Resource for managing an AWS Network Manager Connect Peer.
///
/// ## Example Usage
///
/// ### Basic Usage
///
///
/// ### Usage with attachment accepter
///
///
/// ### Usage with a Tunnel-less Connect attachment
///
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_connect_peer` using the connect peer ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/connectPeer:ConnectPeer example connect-peer-061f3e96275db1acc
/// ```
pub mod connect_peer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectPeerArgs {
        /// The Connect peer BGP options.
        #[builder(into, default)]
        pub bgp_options: pulumi_wasm_rust::Output<
            Option<super::super::types::networkmanager::ConnectPeerBgpOptions>,
        >,
        /// The ID of the connection attachment.
        #[builder(into)]
        pub connect_attachment_id: pulumi_wasm_rust::Output<String>,
        /// A Connect peer core network address.
        #[builder(into, default)]
        pub core_network_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
        #[builder(into, default)]
        pub inside_cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Connect peer address.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub peer_address: pulumi_wasm_rust::Output<String>,
        /// The subnet ARN for the Connect peer. Required when the Connect attachment protocol is `NO_ENCAP`. See `aws.networkmanager.ConnectAttachment` for details.
        #[builder(into, default)]
        pub subnet_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectPeerResult {
        /// The ARN of the attachment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Connect peer BGP options.
        pub bgp_options: pulumi_wasm_rust::Output<
            Option<super::super::types::networkmanager::ConnectPeerBgpOptions>,
        >,
        /// The configuration of the Connect peer.
        pub configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkmanager::ConnectPeerConfiguration>,
        >,
        /// The ID of the connection attachment.
        pub connect_attachment_id: pulumi_wasm_rust::Output<String>,
        pub connect_peer_id: pulumi_wasm_rust::Output<String>,
        /// A Connect peer core network address.
        pub core_network_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of a core network.
        pub core_network_id: pulumi_wasm_rust::Output<String>,
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The Region where the peer is located.
        pub edge_location: pulumi_wasm_rust::Output<String>,
        /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
        pub inside_cidr_blocks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Connect peer address.
        ///
        /// The following arguments are optional:
        pub peer_address: pulumi_wasm_rust::Output<String>,
        /// The state of the Connect peer.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subnet ARN for the Connect peer. Required when the Connect attachment protocol is `NO_ENCAP`. See `aws.networkmanager.ConnectAttachment` for details.
        pub subnet_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectPeerArgs) -> ConnectPeerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bgp_options_binding = args.bgp_options.get_inner();
        let connect_attachment_id_binding = args.connect_attachment_id.get_inner();
        let core_network_address_binding = args.core_network_address.get_inner();
        let inside_cidr_blocks_binding = args.inside_cidr_blocks.get_inner();
        let peer_address_binding = args.peer_address.get_inner();
        let subnet_arn_binding = args.subnet_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/connectPeer:ConnectPeer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bgpOptions".into(),
                    value: &bgp_options_binding,
                },
                register_interface::ObjectField {
                    name: "connectAttachmentId".into(),
                    value: &connect_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "coreNetworkAddress".into(),
                    value: &core_network_address_binding,
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
                    name: "subnetArn".into(),
                    value: &subnet_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bgpOptions".into(),
                },
                register_interface::ResultField {
                    name: "configurations".into(),
                },
                register_interface::ResultField {
                    name: "connectAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "connectPeerId".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkAddress".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "edgeLocation".into(),
                },
                register_interface::ResultField {
                    name: "insideCidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "peerAddress".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
            bgp_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpOptions").unwrap(),
            ),
            configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurations").unwrap(),
            ),
            connect_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectAttachmentId").unwrap(),
            ),
            connect_peer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectPeerId").unwrap(),
            ),
            core_network_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkAddress").unwrap(),
            ),
            core_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkId").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            edge_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeLocation").unwrap(),
            ),
            inside_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insideCidrBlocks").unwrap(),
            ),
            peer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAddress").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
