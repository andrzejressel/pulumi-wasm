/// Resource for managing an AWS Managed Streaming for Kafka VPC Connection.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import MSK configurations using the configuration ARN. For example:
///
/// ```sh
/// $ pulumi import aws:msk/vpcConnection:VpcConnection example arn:aws:kafka:eu-west-2:123456789012:vpc-connection/123456789012/example/38173259-79cd-4ee8-87f3-682ea6023f48-2
/// ```
pub mod vpc_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcConnectionArgs {
        /// The authentication type for the client VPC connection. Specify one of these auth type strings: SASL_IAM, SASL_SCRAM, or TLS.
        #[builder(into)]
        pub authentication: pulumi_wasm_rust::Output<String>,
        /// The list of subnets in the client VPC to connect to.
        #[builder(into)]
        pub client_subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// The security groups to attach to the ENIs for the broker nodes.
        #[builder(into)]
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) of the cluster.
        #[builder(into)]
        pub target_cluster_arn: pulumi_wasm_rust::Output<String>,
        /// The VPC ID of the remote client.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcConnectionResult {
        /// Amazon Resource Name (ARN) of the VPC connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The authentication type for the client VPC connection. Specify one of these auth type strings: SASL_IAM, SASL_SCRAM, or TLS.
        pub authentication: pulumi_wasm_rust::Output<String>,
        /// The list of subnets in the client VPC to connect to.
        pub client_subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// The security groups to attach to the ENIs for the broker nodes.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) of the cluster.
        pub target_cluster_arn: pulumi_wasm_rust::Output<String>,
        /// The VPC ID of the remote client.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcConnectionArgs) -> VpcConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_binding = args.authentication.get_inner();
        let client_subnets_binding = args.client_subnets.get_inner();
        let security_groups_binding = args.security_groups.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_cluster_arn_binding = args.target_cluster_arn.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:msk/vpcConnection:VpcConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "clientSubnets".into(),
                    value: &client_subnets_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetClusterArn".into(),
                    value: &target_cluster_arn_binding,
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
                    name: "authentication".into(),
                },
                register_interface::ResultField {
                    name: "clientSubnets".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetClusterArn".into(),
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
        VpcConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authentication").unwrap(),
            ),
            client_subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSubnets").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetClusterArn").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}