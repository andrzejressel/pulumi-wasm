/// Resource for managing an AWS QuickSight VPC Connection.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   vpcConnectionRole:
///     type: aws:iam:Role
///     name: vpc_connection_role
///     properties:
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Effect: Allow
///               Action: sts:AssumeRole
///               Principal:
///                 Service: quicksight.amazonaws.com
///       inlinePolicies:
///         - name: QuickSightVPCConnectionRolePolicy
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Effect: Allow
///                   Action:
///                     - ec2:CreateNetworkInterface
///                     - ec2:ModifyNetworkInterfaceAttribute
///                     - ec2:DeleteNetworkInterface
///                     - ec2:DescribeSubnets
///                     - ec2:DescribeSecurityGroups
///                   Resource:
///                     - '*'
///   example:
///     type: aws:quicksight:VpcConnection
///     properties:
///       vpcConnectionId: example-connection-id
///       name: Example Connection
///       roleArn: ${vpcConnectionRole.arn}
///       securityGroupIds:
///         - sg-00000000000000000
///       subnetIds:
///         - subnet-00000000000000000
///         - subnet-00000000000000001
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight VPC connection using the AWS account ID and VPC connection ID separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/vpcConnection:VpcConnection example 123456789012,example
/// ```
pub mod vpc_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcConnectionArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IP addresses of DNS resolver endpoints for the VPC connection.
        #[builder(into, default)]
        pub dns_resolvers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The display name for the VPC connection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The IAM role to associate with the VPC connection.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A list of security group IDs for the VPC connection.
        #[builder(into)]
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of subnet IDs for the VPC connection.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::VpcConnectionTimeouts>,
        >,
        /// The ID of the VPC connection.
        #[builder(into)]
        pub vpc_connection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcConnectionResult {
        /// ARN of the VPC connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The availability status of the VPC connection. Valid values are `AVAILABLE`, `UNAVAILABLE` or `PARTIALLY_AVAILABLE`.
        pub availability_status: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// A list of IP addresses of DNS resolver endpoints for the VPC connection.
        pub dns_resolvers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The display name for the VPC connection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The IAM role to associate with the VPC connection.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A list of security group IDs for the VPC connection.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of subnet IDs for the VPC connection.
        ///
        /// The following arguments are optional:
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::VpcConnectionTimeouts>,
        >,
        /// The ID of the VPC connection.
        pub vpc_connection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcConnectionArgs) -> VpcConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let dns_resolvers_binding = args.dns_resolvers.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let vpc_connection_id_binding = args.vpc_connection_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/vpcConnection:VpcConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsResolvers".into(),
                    value: &dns_resolvers_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
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
                    name: "vpcConnectionId".into(),
                    value: &vpc_connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityStatus".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "dnsResolvers".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
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
                    name: "vpcConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityStatus").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            dns_resolvers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsResolvers").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
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
            vpc_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConnectionId").unwrap(),
            ),
        }
    }
}