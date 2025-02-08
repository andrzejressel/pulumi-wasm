/// Manages an AWS DataSync Agent deployed on premises.
///
/// > **NOTE:** One of `activation_key` or `ip_address` must be provided for resource creation (agent activation). Neither is required for resource import. If using `ip_address`, this provider must be able to make an HTTP (port 80) GET request to the specified IP address from where it is running. The agent will turn off that HTTP server after activation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ```yaml
/// resources:
///   exampleAgent:
///     type: aws:datasync:Agent
///     name: example
///     properties:
///       ipAddress: 1.2.3.4
///       securityGroupArns:
///         - ${exampleAwsSecurityGroup.arn}
///       subnetArns:
///         - ${exampleAwsSubnet.arn}
///       vpcEndpointId: ${exampleVpcEndpoint.id}
///       privateLinkEndpoint: ${example.privateIp}
///       name: example
///   exampleVpcEndpoint:
///     type: aws:ec2:VpcEndpoint
///     name: example
///     properties:
///       serviceName: com.amazonaws.${current.name}.datasync
///       vpcId: ${exampleAwsVpc.id}
///       securityGroupIds:
///         - ${exampleAwsSecurityGroup.id}
///       subnetIds:
///         - ${exampleAwsSubnet.id}
///       vpcEndpointType: Interface
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:ec2:getNetworkInterface
///       arguments:
///         id: ${exampleVpcEndpoint.networkInterfaceIds[0]}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_agent` using the DataSync Agent Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/agent:Agent example arn:aws:datasync:us-east-1:123456789012:agent/agent-12345678901234567
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod agent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentArgs {
        /// DataSync Agent activation key during resource creation. Conflicts with `ip_address`. If an `ip_address` is provided instead, the provider will retrieve the `activation_key` as part of the resource creation.
        #[builder(into, default)]
        pub activation_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DataSync Agent IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. DataSync Agent must be accessible on port 80 from where the provider is running.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the DataSync Agent.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP address of the VPC endpoint the agent should connect to when retrieving an activation key during resource creation. Conflicts with `activation_key`.
        #[builder(into, default)]
        pub private_link_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARNs of the security groups used to protect your data transfer task subnets.
        #[builder(into, default)]
        pub security_group_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Amazon Resource Names (ARNs) of the subnets in which DataSync will create elastic network interfaces for each data transfer task.
        #[builder(into, default)]
        pub subnet_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value pairs of resource tags to assign to the DataSync Agent. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the VPC (virtual private cloud) endpoint that the agent has access to.
        #[builder(into, default)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AgentResult {
        /// DataSync Agent activation key during resource creation. Conflicts with `ip_address`. If an `ip_address` is provided instead, the provider will retrieve the `activation_key` as part of the resource creation.
        pub activation_key: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the DataSync Agent.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// DataSync Agent IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. DataSync Agent must be accessible on port 80 from where the provider is running.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// Name of the DataSync Agent.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The IP address of the VPC endpoint the agent should connect to when retrieving an activation key during resource creation. Conflicts with `activation_key`.
        pub private_link_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The ARNs of the security groups used to protect your data transfer task subnets.
        pub security_group_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Amazon Resource Names (ARNs) of the subnets in which DataSync will create elastic network interfaces for each data transfer task.
        pub subnet_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Key-value pairs of resource tags to assign to the DataSync Agent. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the VPC (virtual private cloud) endpoint that the agent has access to.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AgentArgs,
    ) -> AgentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activation_key_binding = args.activation_key.get_output(context).get_inner();
        let ip_address_binding = args.ip_address.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_link_endpoint_binding = args
            .private_link_endpoint
            .get_output(context)
            .get_inner();
        let security_group_arns_binding = args
            .security_group_arns
            .get_output(context)
            .get_inner();
        let subnet_arns_binding = args.subnet_arns.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_endpoint_id_binding = args
            .vpc_endpoint_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/agent:Agent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AgentResult {
            activation_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activationKey"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_link_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLinkEndpoint"),
            ),
            security_group_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupArns"),
            ),
            subnet_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetArns"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcEndpointId"),
            ),
        }
    }
}
