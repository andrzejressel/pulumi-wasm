/// Creates a transit gateway route table attachment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway_route_table_attachment::create(
///         "example",
///         TransitGatewayRouteTableAttachmentArgs::builder()
///             .peering_id("${exampleAwsNetworkmanagerTransitGatewayPeering.id}")
///             .transit_gateway_route_table_arn(
///                 "${exampleAwsEc2TransitGatewayRouteTable.arn}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_transit_gateway_route_table_attachment` using the attachment ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/transitGatewayRouteTableAttachment:TransitGatewayRouteTableAttachment example attachment-0f8fa60d2238d1bd8
/// ```
pub mod transit_gateway_route_table_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayRouteTableAttachmentArgs {
        /// The ID of the peer for the attachment.
        #[builder(into)]
        pub peering_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the transit gateway route table for the attachment.
        #[builder(into)]
        pub transit_gateway_route_table_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayRouteTableAttachmentResult {
        /// Attachment Amazon Resource Name (ARN).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_wasm_rust::Output<i32>,
        /// The type of attachment.
        pub attachment_type: pulumi_wasm_rust::Output<String>,
        /// The ARN of the core network.
        pub core_network_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the core network.
        pub core_network_id: pulumi_wasm_rust::Output<String>,
        /// The edge location for the peer.
        pub edge_location: pulumi_wasm_rust::Output<String>,
        /// The ID of the attachment account owner.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the peer for the attachment.
        pub peering_id: pulumi_wasm_rust::Output<String>,
        /// The attachment resource ARN.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the segment attachment.
        pub segment_name: pulumi_wasm_rust::Output<String>,
        /// The state of the attachment.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the transit gateway route table for the attachment.
        pub transit_gateway_route_table_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TransitGatewayRouteTableAttachmentArgs,
    ) -> TransitGatewayRouteTableAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let peering_id_binding = args.peering_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_route_table_arn_binding = args
            .transit_gateway_route_table_arn
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayRouteTableAttachment:TransitGatewayRouteTableAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "peeringId".into(),
                    value: &peering_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayRouteTableArn".into(),
                    value: &transit_gateway_route_table_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attachmentPolicyRuleNumber".into(),
                },
                register_interface::ResultField {
                    name: "attachmentType".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkArn".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "edgeLocation".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "peeringId".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "segmentName".into(),
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
                    name: "transitGatewayRouteTableArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TransitGatewayRouteTableAttachmentResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attachment_policy_rule_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentPolicyRuleNumber").unwrap(),
            ),
            attachment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentType").unwrap(),
            ),
            core_network_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkArn").unwrap(),
            ),
            core_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkId").unwrap(),
            ),
            edge_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeLocation").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
            ),
            peering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peeringId").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            segment_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("segmentName").unwrap(),
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
            transit_gateway_route_table_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayRouteTableArn").unwrap(),
            ),
        }
    }
}
