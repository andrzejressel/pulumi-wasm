/// Provides a resource to create a VPC Internet Gateway Attachment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = internet_gateway_attachment::create(
///         "example",
///         InternetGatewayAttachmentArgs::builder()
///             .internet_gateway_id("${exampleInternetGateway.id}")
///             .vpc_id("${exampleVpc.id}")
///             .build_struct(),
///     );
///     let exampleInternetGateway = internet_gateway::create(
///         "exampleInternetGateway",
///         InternetGatewayArgs::builder().build_struct(),
///     );
///     let exampleVpc = vpc::create(
///         "exampleVpc",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Internet Gateway Attachments using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/internetGatewayAttachment:InternetGatewayAttachment example igw-c0a643a9:vpc-123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod internet_gateway_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InternetGatewayAttachmentArgs {
        /// The ID of the internet gateway.
        #[builder(into)]
        pub internet_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InternetGatewayAttachmentResult {
        /// The ID of the internet gateway.
        pub internet_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InternetGatewayAttachmentArgs,
    ) -> InternetGatewayAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let internet_gateway_id_binding = args.internet_gateway_id.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/internetGatewayAttachment:InternetGatewayAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetGatewayId".into(),
                    value: internet_gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InternetGatewayAttachmentResult {
            internet_gateway_id: o.get_field("internetGatewayId"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
