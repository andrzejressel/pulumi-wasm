/// Manages an AWS DataSync Agent deployed on premises.
///
/// > **NOTE:** One of `activation_key` or `ip_address` must be provided for resource creation (agent activation). Neither is required for resource import. If using `ip_address`, this provider must be able to make an HTTP (port 80) GET request to the specified IP address from where it is running. The agent will turn off that HTTP server after activation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = agent::create(
///         "example",
///         AgentArgs::builder().ip_address("1.2.3.4").name("example").build_struct(),
///     );
/// }
/// ```
///
///
/// ### With VPC Endpoints
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let example = get_network_interface::invoke(
///         GetNetworkInterfaceArgs::builder()
///             .id("${exampleVpcEndpoint.networkInterfaceIds[0]}")
///             .build_struct(),
///     );
///     let exampleAgent = agent::create(
///         "exampleAgent",
///         AgentArgs::builder()
///             .ip_address("1.2.3.4")
///             .name("example")
///             .private_link_endpoint("${example.privateIp}")
///             .security_group_arns(vec!["${exampleAwsSecurityGroup.arn}",])
///             .subnet_arns(vec!["${exampleAwsSubnet.arn}",])
///             .vpc_endpoint_id("${exampleVpcEndpoint.id}")
///             .build_struct(),
///     );
///     let exampleVpcEndpoint = vpc_endpoint::create(
///         "exampleVpcEndpoint",
///         VpcEndpointArgs::builder()
///             .security_group_ids(vec!["${exampleAwsSecurityGroup.id}",])
///             .service_name("com.amazonaws.${current.name}.datasync")
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .vpc_endpoint_type("Interface")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_agent` using the DataSync Agent Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/agent:Agent example arn:aws:datasync:us-east-1:123456789012:agent/agent-12345678901234567
/// ```
pub mod agent {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentArgs {
        /// DataSync Agent activation key during resource creation. Conflicts with `ip_address`. If an `ip_address` is provided instead, the provider will retrieve the `activation_key` as part of the resource creation.
        #[builder(into, default)]
        pub activation_key: pulumi_wasm_rust::Output<Option<String>>,
        /// DataSync Agent IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. DataSync Agent must be accessible on port 80 from where the provider is running.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the DataSync Agent.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP address of the VPC endpoint the agent should connect to when retrieving an activation key during resource creation. Conflicts with `activation_key`.
        #[builder(into, default)]
        pub private_link_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARNs of the security groups used to protect your data transfer task subnets.
        #[builder(into, default)]
        pub security_group_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Amazon Resource Names (ARNs) of the subnets in which DataSync will create elastic network interfaces for each data transfer task.
        #[builder(into, default)]
        pub subnet_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value pairs of resource tags to assign to the DataSync Agent. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the VPC (virtual private cloud) endpoint that the agent has access to.
        #[builder(into, default)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AgentResult {
        /// DataSync Agent activation key during resource creation. Conflicts with `ip_address`. If an `ip_address` is provided instead, the provider will retrieve the `activation_key` as part of the resource creation.
        pub activation_key: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the DataSync Agent.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// DataSync Agent IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. DataSync Agent must be accessible on port 80 from where the provider is running.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// Name of the DataSync Agent.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The IP address of the VPC endpoint the agent should connect to when retrieving an activation key during resource creation. Conflicts with `activation_key`.
        pub private_link_endpoint: pulumi_wasm_rust::Output<String>,
        /// The ARNs of the security groups used to protect your data transfer task subnets.
        pub security_group_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Amazon Resource Names (ARNs) of the subnets in which DataSync will create elastic network interfaces for each data transfer task.
        pub subnet_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-value pairs of resource tags to assign to the DataSync Agent. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC (virtual private cloud) endpoint that the agent has access to.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AgentArgs) -> AgentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activation_key_binding = args.activation_key.get_inner();
        let ip_address_binding = args.ip_address.get_inner();
        let name_binding = args.name.get_inner();
        let private_link_endpoint_binding = args.private_link_endpoint.get_inner();
        let security_group_arns_binding = args.security_group_arns.get_inner();
        let subnet_arns_binding = args.subnet_arns.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/agent:Agent".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activationKey".into(),
                    value: &activation_key_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkEndpoint".into(),
                    value: &private_link_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupArns".into(),
                    value: &security_group_arns_binding,
                },
                register_interface::ObjectField {
                    name: "subnetArns".into(),
                    value: &subnet_arns_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activationKey".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupArns".into(),
                },
                register_interface::ResultField {
                    name: "subnetArns".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AgentResult {
            activation_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activationKey").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_link_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkEndpoint").unwrap(),
            ),
            security_group_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupArns").unwrap(),
            ),
            subnet_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetArns").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
        }
    }
}