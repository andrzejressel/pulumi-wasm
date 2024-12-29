/// Manages an EC2 Transit Gateway Peering Attachment.
/// For examples of custom route table association and propagation, see the [EC2 Transit Gateway Networking Examples Guide](https://docs.aws.amazon.com/vpc/latest/tgw/TGW_Scenarios.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   local:
///     type: aws:ec2transitgateway:TransitGateway
///     properties:
///       tags:
///         Name: Local TGW
///   peerTransitGateway:
///     type: aws:ec2transitgateway:TransitGateway
///     name: peer
///     properties:
///       tags:
///         Name: Peer TGW
///   example:
///     type: aws:ec2transitgateway:PeeringAttachment
///     properties:
///       peerAccountId: ${peerTransitGateway.ownerId}
///       peerRegion: ${peer.name}
///       peerTransitGatewayId: ${peerTransitGateway.id}
///       transitGatewayId: ${local.id}
///       tags:
///         Name: TGW Peering Requestor
/// variables:
///   peer:
///     fn::invoke:
///       Function: aws:getRegion
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_peering_attachment` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/peeringAttachment:PeeringAttachment example tgw-attach-12345678
/// ```
pub mod peering_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringAttachmentArgs {
        /// Describes whether dynamic routing is enabled or disabled for the transit gateway peering request. See options below for more details!
        #[builder(into, default)]
        pub options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2transitgateway::PeeringAttachmentOptions>,
        >,
        /// Account ID of EC2 Transit Gateway to peer with. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub peer_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Region of EC2 Transit Gateway to peer with.
        #[builder(into)]
        pub peer_region: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway to peer with.
        #[builder(into)]
        pub peer_transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of EC2 Transit Gateway.
        #[builder(into)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PeeringAttachmentResult {
        /// Describes whether dynamic routing is enabled or disabled for the transit gateway peering request. See options below for more details!
        pub options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2transitgateway::PeeringAttachmentOptions>,
        >,
        /// Account ID of EC2 Transit Gateway to peer with. Defaults to the account ID the AWS provider is currently connected to.
        pub peer_account_id: pulumi_wasm_rust::Output<String>,
        /// Region of EC2 Transit Gateway to peer with.
        pub peer_region: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway to peer with.
        pub peer_transit_gateway_id: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PeeringAttachmentArgs) -> PeeringAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let options_binding = args.options.get_inner();
        let peer_account_id_binding = args.peer_account_id.get_inner();
        let peer_region_binding = args.peer_region.get_inner();
        let peer_transit_gateway_id_binding = args.peer_transit_gateway_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/peeringAttachment:PeeringAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
                register_interface::ObjectField {
                    name: "peerAccountId".into(),
                    value: &peer_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "peerRegion".into(),
                    value: &peer_region_binding,
                },
                register_interface::ObjectField {
                    name: "peerTransitGatewayId".into(),
                    value: &peer_transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "options".into(),
                },
                register_interface::ResultField {
                    name: "peerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "peerRegion".into(),
                },
                register_interface::ResultField {
                    name: "peerTransitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PeeringAttachmentResult {
            options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("options").unwrap(),
            ),
            peer_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAccountId").unwrap(),
            ),
            peer_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerRegion").unwrap(),
            ),
            peer_transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerTransitGatewayId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
        }
    }
}
