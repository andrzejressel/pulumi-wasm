/// Creates a peering connection between an AWS Cloud WAN core network and an AWS Transit Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway_peering::create(
///         "example",
///         TransitGatewayPeeringArgs::builder()
///             .core_network_id("${exampleAwsccNetworkmanagerCoreNetwork.id}")
///             .transit_gateway_arn("${exampleAwsEc2TransitGateway.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_transit_gateway_peering` using the peering ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/transitGatewayPeering:TransitGatewayPeering example peering-444555aaabbb11223
/// ```
pub mod transit_gateway_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayPeeringArgs {
        /// The ID of a core network.
        #[builder(into)]
        pub core_network_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value tags for the peering. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the transit gateway for the peering request.
        #[builder(into)]
        pub transit_gateway_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayPeeringResult {
        /// Peering Amazon Resource Name (ARN).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the core network.
        pub core_network_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of a core network.
        pub core_network_id: pulumi_wasm_rust::Output<String>,
        /// The edge location for the peer.
        pub edge_location: pulumi_wasm_rust::Output<String>,
        /// The ID of the account owner.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The type of peering. This will be `TRANSIT_GATEWAY`.
        pub peering_type: pulumi_wasm_rust::Output<String>,
        /// The resource ARN of the peer.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the peering. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the transit gateway for the peering request.
        pub transit_gateway_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the transit gateway peering attachment.
        pub transit_gateway_peering_attachment_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TransitGatewayPeeringArgs,
    ) -> TransitGatewayPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let core_network_id_binding = args
            .core_network_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_arn_binding = args
            .transit_gateway_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayPeering:TransitGatewayPeering"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "coreNetworkId".into(),
                    value: &core_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayArn".into(),
                    value: &transit_gateway_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
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
                    name: "peeringType".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayArn".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayPeeringAttachmentId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TransitGatewayPeeringResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
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
            peering_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peeringType").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayArn").unwrap(),
            ),
            transit_gateway_peering_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayPeeringAttachmentId").unwrap(),
            ),
        }
    }
}
