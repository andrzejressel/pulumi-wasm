/// Resource for managing an AWS OpenSearchServerless VPC Endpoint.
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
///     let example = serverless_vpc_endpoint::create(
///         "example",
///         ServerlessVpcEndpointArgs::builder()
///             .name("myendpoint")
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Vpc Endpointa using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessVpcEndpoint:ServerlessVpcEndpoint example vpce-8012925589
/// ```
pub mod serverless_vpc_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessVpcEndpointArgs {
        /// Name of the interface endpoint.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint. Up to 5 security groups can be provided.
        #[builder(into, default)]
        pub security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more subnet IDs from which you'll access OpenSearch Serverless. Up to 6 subnets can be provided.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::ServerlessVpcEndpointTimeouts>,
        >,
        /// ID of the VPC from which you'll access OpenSearch Serverless.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessVpcEndpointResult {
        /// Name of the interface endpoint.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint. Up to 5 security groups can be provided.
        pub security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more subnet IDs from which you'll access OpenSearch Serverless. Up to 6 subnets can be provided.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::ServerlessVpcEndpointTimeouts>,
        >,
        /// ID of the VPC from which you'll access OpenSearch Serverless.
        ///
        /// The following arguments are optional:
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServerlessVpcEndpointArgs,
    ) -> ServerlessVpcEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let security_group_ids_binding = args.security_group_ids.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessVpcEndpoint:ServerlessVpcEndpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
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
        ServerlessVpcEndpointResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupIds").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
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