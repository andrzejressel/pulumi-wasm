/// Provides a resource to create a VPC Internet Gateway Attachment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod internet_gateway_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InternetGatewayAttachmentArgs {
        /// The ID of the internet gateway.
        #[builder(into)]
        pub internet_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InternetGatewayAttachmentResult {
        /// The ID of the internet gateway.
        pub internet_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InternetGatewayAttachmentArgs,
    ) -> InternetGatewayAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let internet_gateway_id_binding = args.internet_gateway_id.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/internetGatewayAttachment:InternetGatewayAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "internetGatewayId".into(),
                    value: &internet_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "internetGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InternetGatewayAttachmentResult {
            internet_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetGatewayId").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}