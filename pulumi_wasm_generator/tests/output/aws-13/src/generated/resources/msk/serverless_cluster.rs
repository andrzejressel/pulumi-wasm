/// Manages an Amazon MSK Serverless cluster.
///
/// > **Note:** To manage a _provisioned_ Amazon MSK cluster, use the `aws.msk.Cluster` resource.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import MSK serverless clusters using the cluster `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:msk/serverlessCluster:ServerlessCluster example arn:aws:kafka:us-west-2:123456789012:cluster/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
pub mod serverless_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessClusterArgs {
        /// Specifies client authentication information for the serverless cluster. See below.
        #[builder(into)]
        pub client_authentication: pulumi_wasm_rust::InputOrOutput<
            super::super::types::msk::ServerlessClusterClientAuthentication,
        >,
        /// The name of the serverless cluster.
        #[builder(into, default)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC configuration information. See below.
        #[builder(into)]
        pub vpc_configs: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::msk::ServerlessClusterVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerlessClusterResult {
        /// The ARN of the serverless cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies client authentication information for the serverless cluster. See below.
        pub client_authentication: pulumi_wasm_rust::Output<
            super::super::types::msk::ServerlessClusterClientAuthentication,
        >,
        /// The name of the serverless cluster.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// UUID of the serverless cluster, for use in IAM policies.
        pub cluster_uuid: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// VPC configuration information. See below.
        pub vpc_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::msk::ServerlessClusterVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerlessClusterArgs,
    ) -> ServerlessClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_authentication_binding = args
            .client_authentication
            .get_output(context)
            .get_inner();
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_configs_binding = args.vpc_configs.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:msk/serverlessCluster:ServerlessCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientAuthentication".into(),
                    value: &client_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfigs".into(),
                    value: &vpc_configs_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clientAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "clusterUuid".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfigs".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerlessClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            client_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientAuthentication").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            cluster_uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterUuid").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfigs").unwrap(),
            ),
        }
    }
}
