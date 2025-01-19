/// Provides a resource to create a VPC Internet Gateway.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   gw:
///     type: aws:ec2:InternetGateway
///     properties:
///       vpcId: ${main.id}
///       tags:
///         Name: main
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Internet Gateways using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/internetGateway:InternetGateway gw igw-c0a643a9
/// ```
pub mod internet_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InternetGatewayArgs {
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **Note:** It's recommended to denote that the AWS Instance or Elastic IP depends on the Internet Gateway. For example:
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let foo = instance::create("foo", InstanceArgs::builder().build_struct());
        ///     let gw = internet_gateway::create(
        ///         "gw",
        ///         InternetGatewayArgs::builder().vpc_id("${main.id}").build_struct(),
        ///     );
        /// }
        /// ```
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC ID to create in.  See the aws.ec2.InternetGatewayAttachment resource for an alternate way to attach an Internet Gateway to a VPC.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InternetGatewayResult {
        /// The ARN of the Internet Gateway.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the internet gateway.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **Note:** It's recommended to denote that the AWS Instance or Elastic IP depends on the Internet Gateway. For example:
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let foo = instance::create("foo", InstanceArgs::builder().build_struct());
        ///     let gw = internet_gateway::create(
        ///         "gw",
        ///         InternetGatewayArgs::builder().vpc_id("${main.id}").build_struct(),
        ///     );
        /// }
        /// ```
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC ID to create in.  See the aws.ec2.InternetGatewayAttachment resource for an alternate way to attach an Internet Gateway to a VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InternetGatewayArgs) -> InternetGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/internetGateway:InternetGateway".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
        InternetGatewayResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
