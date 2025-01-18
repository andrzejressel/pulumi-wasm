/// Resource for managing an AWS Network Manager Attachment Accepter.
///
/// ## Example Usage
///
/// ### Example with VPC attachment
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = attachment_accepter::create(
///         "test",
///         AttachmentAccepterArgs::builder()
///             .attachment_id("${vpc.id}")
///             .attachment_type("${vpc.attachmentType}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example with site-to-site VPN attachment
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = attachment_accepter::create(
///         "test",
///         AttachmentAccepterArgs::builder()
///             .attachment_id("${vpn.id}")
///             .attachment_type("${vpn.attachmentType}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod attachment_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachmentAccepterArgs {
        /// The ID of the attachment.
        #[builder(into)]
        pub attachment_id: pulumi_wasm_rust::Output<String>,
        /// The type of attachment. Valid values can be found in the [AWS Documentation](https://docs.aws.amazon.com/networkmanager/latest/APIReference/API_ListAttachments.html#API_ListAttachments_RequestSyntax)
        #[builder(into)]
        pub attachment_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AttachmentAccepterResult {
        /// The ID of the attachment.
        pub attachment_id: pulumi_wasm_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_wasm_rust::Output<i32>,
        /// The type of attachment. Valid values can be found in the [AWS Documentation](https://docs.aws.amazon.com/networkmanager/latest/APIReference/API_ListAttachments.html#API_ListAttachments_RequestSyntax)
        pub attachment_type: pulumi_wasm_rust::Output<String>,
        /// The ARN of a core network.
        pub core_network_arn: pulumi_wasm_rust::Output<String>,
        /// The id of a core network.
        pub core_network_id: pulumi_wasm_rust::Output<String>,
        /// The Region where the edge is located. This is returned for all attachment types except a Direct Connect gateway attachment, which instead returns `edge_locations`.
        pub edge_location: pulumi_wasm_rust::Output<String>,
        /// The edge locations that the Direct Connect gateway is associated with. This is returned only for Direct Connect gateway attachments. All other attachment types return `edge_location`
        pub edge_locations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the attachment account owner.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The attachment resource ARN.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the segment attachment.
        pub segment_name: pulumi_wasm_rust::Output<String>,
        /// The state of the attachment.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AttachmentAccepterArgs) -> AttachmentAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachment_id_binding = args.attachment_id.get_inner();
        let attachment_type_binding = args.attachment_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/attachmentAccepter:AttachmentAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachmentId".into(),
                    value: &attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "attachmentType".into(),
                    value: &attachment_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attachmentId".into(),
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
                    name: "edgeLocations".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AttachmentAccepterResult {
            attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentId").unwrap(),
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
            edge_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeLocations").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
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
        }
    }
}
