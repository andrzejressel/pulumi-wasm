/// Manages an EC2 Carrier Gateway. See the AWS [documentation](https://docs.aws.amazon.com/vpc/latest/userguide/Carrier_Gateway.html) for more information.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:CarrierGateway
///     properties:
///       vpcId: ${exampleAwsVpc.id}
///       tags:
///         Name: example-carrier-gateway
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_carrier_gateway` using the carrier gateway's ID. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/carrierGateway:CarrierGateway example cgw-12345
/// ```
pub mod carrier_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CarrierGatewayArgs {
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the VPC to associate with the carrier gateway.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CarrierGatewayResult {
        /// The ARN of the carrier gateway.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID of the owner of the carrier gateway.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC to associate with the carrier gateway.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CarrierGatewayArgs,
    ) -> CarrierGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/carrierGateway:CarrierGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CarrierGatewayResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
