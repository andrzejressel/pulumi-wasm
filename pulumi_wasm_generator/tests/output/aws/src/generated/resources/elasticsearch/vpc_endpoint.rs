/// Manages an [AWS Elasticsearch VPC Endpoint](https://docs.aws.amazon.com/elasticsearch-service/latest/APIReference/API_CreateVpcEndpoint.html). Creates an Amazon elasticsearch Service-managed VPC endpoint.
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
///     let foo = vpc_endpoint::create(
///         "foo",
///         VpcEndpointArgs::builder()
///             .domain_arn("${domain1.arn}")
///             .vpc_options(
///                 VpcEndpointVpcOptions::builder()
///                     .securityGroupIds(vec!["${test.id}", "${test2.id}",])
///                     .subnetIds(vec!["${testAwsSubnet.id}", "${test2AwsSubnet.id}",])
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import elasticsearch VPC endpoint connections using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticsearch/vpcEndpoint:VpcEndpoint example endpoint-id
/// ```
pub mod vpc_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointArgs {
        /// Specifies the Amazon Resource Name (ARN) of the domain to create the endpoint for
        #[builder(into)]
        pub domain_arn: pulumi_wasm_rust::Output<String>,
        /// Options to specify the subnets and security groups for the endpoint.
        #[builder(into)]
        pub vpc_options: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::VpcEndpointVpcOptions,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointResult {
        /// Specifies the Amazon Resource Name (ARN) of the domain to create the endpoint for
        pub domain_arn: pulumi_wasm_rust::Output<String>,
        /// The connection endpoint ID for connecting to the domain.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Options to specify the subnets and security groups for the endpoint.
        pub vpc_options: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::VpcEndpointVpcOptions,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcEndpointArgs) -> VpcEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_arn_binding = args.domain_arn.get_inner();
        let vpc_options_binding = args.vpc_options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticsearch/vpcEndpoint:VpcEndpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainArn".into(),
                    value: &domain_arn_binding,
                },
                register_interface::ObjectField {
                    name: "vpcOptions".into(),
                    value: &vpc_options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainArn".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "vpcOptions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointResult {
            domain_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainArn").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            vpc_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOptions").unwrap(),
            ),
        }
    }
}