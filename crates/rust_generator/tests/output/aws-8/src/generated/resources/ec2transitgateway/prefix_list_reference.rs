/// Manages an EC2 Transit Gateway Prefix List Reference.
///
/// ## Example Usage
///
/// ### Attachment Routing
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod prefix_list_reference {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrefixListReferenceArgs {
        /// Indicates whether to drop traffic that matches the Prefix List. Defaults to `false`.
        #[builder(into, default)]
        pub blackhole: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Identifier of EC2 Prefix List.
        #[builder(into)]
        pub prefix_list_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PrefixListReferenceResult {
        /// Indicates whether to drop traffic that matches the Prefix List. Defaults to `false`.
        pub blackhole: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Identifier of EC2 Prefix List.
        pub prefix_list_id: pulumi_gestalt_rust::Output<String>,
        pub prefix_list_owner_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// The following arguments are optional:
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrefixListReferenceArgs,
    ) -> PrefixListReferenceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let blackhole_binding = args.blackhole.get_output(context);
        let prefix_list_id_binding = args.prefix_list_id.get_output(context);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/prefixListReference:PrefixListReference"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blackhole".into(),
                    value: blackhole_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefixListId".into(),
                    value: prefix_list_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: transit_gateway_attachment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: transit_gateway_route_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrefixListReferenceResult {
            blackhole: o.get_field("blackhole"),
            prefix_list_id: o.get_field("prefixListId"),
            prefix_list_owner_id: o.get_field("prefixListOwnerId"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}
