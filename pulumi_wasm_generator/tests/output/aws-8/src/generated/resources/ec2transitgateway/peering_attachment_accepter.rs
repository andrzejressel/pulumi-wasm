/// Manages the accepter's side of an EC2 Transit Gateway Peering Attachment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:PeeringAttachmentAccepter
///     properties:
///       transitGatewayAttachmentId: ${exampleAwsEc2TransitGatewayPeeringAttachment.id}
///       tags:
///         Name: Example cross-account attachment
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_peering_attachment_accepter` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/peeringAttachmentAccepter:PeeringAttachmentAccepter example tgw-attach-12345678
/// ```
pub mod peering_attachment_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringAttachmentAccepterArgs {
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the EC2 Transit Gateway Peering Attachment to manage.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PeeringAttachmentAccepterResult {
        /// Identifier of the AWS account that owns the EC2 TGW peering.
        pub peer_account_id: pulumi_wasm_rust::Output<String>,
        pub peer_region: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway to peer with.
        pub peer_transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the EC2 Transit Gateway Peering Attachment to manage.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PeeringAttachmentAccepterArgs,
    ) -> PeeringAttachmentAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/peeringAttachmentAccepter:PeeringAttachmentAccepter"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
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
        PeeringAttachmentAccepterResult {
            peer_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAccountId").unwrap(),
            ),
            peer_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerRegion").unwrap(),
            ),
            peer_transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerTransitGatewayId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
        }
    }
}
