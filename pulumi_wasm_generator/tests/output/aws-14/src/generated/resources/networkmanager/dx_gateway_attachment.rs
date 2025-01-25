/// Resource for managing an AWS Network Manager Direct Connect (DX) Gateway Attachment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = dx_gateway_attachment::create(
///         "test",
///         DxGatewayAttachmentArgs::builder()
///             .core_network_id(
///                 "${testAwsNetworkmanagerCoreNetworkPolicyAttachment.coreNetworkId}",
///             )
///             .direct_connect_gateway_arn(
///                 "arn:aws:directconnect::${current.accountId}:dx-gateway/${testAwsDxGateway.id}",
///             )
///             .edge_locations(vec!["${currentAwsRegion.name}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Manager DX Gateway Attachment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/dxGatewayAttachment:DxGatewayAttachment example attachment-1a2b3c4d5e6f7g
/// ```
pub mod dx_gateway_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DxGatewayAttachmentArgs {
        /// ID of the Cloud WAN core network to which the Direct Connect gateway attachment should be attached.
        #[builder(into)]
        pub core_network_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the Direct Connect gateway attachment.
        #[builder(into)]
        pub direct_connect_gateway_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more core network edge locations to associate with the Direct Connect gateway attachment.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub edge_locations: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::networkmanager::DxGatewayAttachmentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DxGatewayAttachmentResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_wasm_rust::Output<i32>,
        /// Type of attachment.
        pub attachment_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the core network for the attachment.
        pub core_network_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Cloud WAN core network to which the Direct Connect gateway attachment should be attached.
        pub core_network_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the Direct Connect gateway attachment.
        pub direct_connect_gateway_arn: pulumi_wasm_rust::Output<String>,
        /// One or more core network edge locations to associate with the Direct Connect gateway attachment.
        ///
        /// The following arguments are optional:
        pub edge_locations: pulumi_wasm_rust::Output<Vec<String>>,
        /// ID of the attachment account owner.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// Name of the segment attachment.
        pub segment_name: pulumi_wasm_rust::Output<String>,
        /// State of the attachment.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::networkmanager::DxGatewayAttachmentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DxGatewayAttachmentArgs,
    ) -> DxGatewayAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let core_network_id_binding = args
            .core_network_id
            .get_output(context)
            .get_inner();
        let direct_connect_gateway_arn_binding = args
            .direct_connect_gateway_arn
            .get_output(context)
            .get_inner();
        let edge_locations_binding = args.edge_locations.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/dxGatewayAttachment:DxGatewayAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "coreNetworkId".into(),
                    value: &core_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "directConnectGatewayArn".into(),
                    value: &direct_connect_gateway_arn_binding,
                },
                register_interface::ObjectField {
                    name: "edgeLocations".into(),
                    value: &edge_locations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
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
                    name: "directConnectGatewayArn".into(),
                },
                register_interface::ResultField {
                    name: "edgeLocations".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
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
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DxGatewayAttachmentResult {
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
            direct_connect_gateway_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directConnectGatewayArn").unwrap(),
            ),
            edge_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeLocations").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
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
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
