/// Manages an EC2 Transit Gateway Prefix List Reference.
///
/// ## Example Usage
///
/// ### Attachment Routing
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = prefix_list_reference::create(
///         "example",
///         PrefixListReferenceArgs::builder()
///             .prefix_list_id("${exampleAwsEc2ManagedPrefixList.id}")
///             .transit_gateway_attachment_id(
///                 "${exampleAwsEc2TransitGatewayVpcAttachment.id}",
///             )
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGateway.associationDefaultRouteTableId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Blackhole Routing
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = prefix_list_reference::create(
///         "example",
///         PrefixListReferenceArgs::builder()
///             .blackhole(true)
///             .prefix_list_id("${exampleAwsEc2ManagedPrefixList.id}")
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGateway.associationDefaultRouteTableId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_prefix_list_reference` using the EC2 Transit Gateway Route Table identifier and EC2 Prefix List identifier, separated by an underscore (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/prefixListReference:PrefixListReference example tgw-rtb-12345678_pl-12345678
/// ```
pub mod prefix_list_reference {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrefixListReferenceArgs {
        /// Indicates whether to drop traffic that matches the Prefix List. Defaults to `false`.
        #[builder(into, default)]
        pub blackhole: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of EC2 Prefix List.
        #[builder(into)]
        pub prefix_list_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PrefixListReferenceResult {
        /// Indicates whether to drop traffic that matches the Prefix List. Defaults to `false`.
        pub blackhole: pulumi_wasm_rust::Output<Option<bool>>,
        /// Identifier of EC2 Prefix List.
        pub prefix_list_id: pulumi_wasm_rust::Output<String>,
        pub prefix_list_owner_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        pub transit_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PrefixListReferenceArgs,
    ) -> PrefixListReferenceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blackhole_binding = args.blackhole.get_inner();
        let prefix_list_id_binding = args.prefix_list_id.get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/prefixListReference:PrefixListReference"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blackhole".into(),
                    value: &blackhole_binding,
                },
                register_interface::ObjectField {
                    name: "prefixListId".into(),
                    value: &prefix_list_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: &transit_gateway_route_table_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blackhole".into(),
                },
                register_interface::ResultField {
                    name: "prefixListId".into(),
                },
                register_interface::ResultField {
                    name: "prefixListOwnerId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayRouteTableId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PrefixListReferenceResult {
            blackhole: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blackhole").unwrap(),
            ),
            prefix_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixListId").unwrap(),
            ),
            prefix_list_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixListOwnerId").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayRouteTableId").unwrap(),
            ),
        }
    }
}
