/// Resource for managing an AWS EC2 (Elastic Compute Cloud) VPC Block Public Access Exclusion.
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
///     let test = vpc::create(
///         "test",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
///     let testVpcBlockPublicAccessExclusion = vpc_block_public_access_exclusion::create(
///         "testVpcBlockPublicAccessExclusion",
///         VpcBlockPublicAccessExclusionArgs::builder()
///             .internet_gateway_exclusion_mode("allow-bidirectional")
///             .vpc_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with subnet id
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = vpc::create(
///         "test",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
///     let testSubnet = subnet::create(
///         "testSubnet",
///         SubnetArgs::builder()
///             .cidr_block("10.1.1.0/24")
///             .vpc_id("${test.id}")
///             .build_struct(),
///     );
///     let testVpcBlockPublicAccessExclusion = vpc_block_public_access_exclusion::create(
///         "testVpcBlockPublicAccessExclusion",
///         VpcBlockPublicAccessExclusionArgs::builder()
///             .internet_gateway_exclusion_mode("allow-egress")
///             .subnet_id("${testSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 (Elastic Compute Cloud) VPC Block Public Access Exclusion using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcBlockPublicAccessExclusion:VpcBlockPublicAccessExclusion example vpcbpa-exclude-1234abcd
/// ```
pub mod vpc_block_public_access_exclusion {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessExclusionArgs {
        /// Mode of exclusion from Block Public Access. The allowed values are `allow-egress` and `allow-bidirectional`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub internet_gateway_exclusion_mode: pulumi_wasm_rust::Output<String>,
        /// Id of the subnet to which this exclusion applies. Either this or the vpc_id needs to be provided.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the exclusion. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcBlockPublicAccessExclusionTimeouts>,
        >,
        /// Id of the VPC to which this exclusion applies. Either this or the subnet_id needs to be provided.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessExclusionResult {
        /// Mode of exclusion from Block Public Access. The allowed values are `allow-egress` and `allow-bidirectional`.
        ///
        /// The following arguments are optional:
        pub internet_gateway_exclusion_mode: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) the excluded resource.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// Id of the subnet to which this exclusion applies. Either this or the vpc_id needs to be provided.
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the exclusion. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcBlockPublicAccessExclusionTimeouts>,
        >,
        /// Id of the VPC to which this exclusion applies. Either this or the subnet_id needs to be provided.
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcBlockPublicAccessExclusionArgs,
    ) -> VpcBlockPublicAccessExclusionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let internet_gateway_exclusion_mode_binding = args
            .internet_gateway_exclusion_mode
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcBlockPublicAccessExclusion:VpcBlockPublicAccessExclusion"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "internetGatewayExclusionMode".into(),
                    value: &internet_gateway_exclusion_mode_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "internetGatewayExclusionMode".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
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
        VpcBlockPublicAccessExclusionResult {
            internet_gateway_exclusion_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetGatewayExclusionMode").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
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
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
