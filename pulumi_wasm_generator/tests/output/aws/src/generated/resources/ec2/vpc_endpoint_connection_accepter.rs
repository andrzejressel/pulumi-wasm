/// Provides a resource to accept a pending VPC Endpoint Connection accept request to VPC Endpoint Service.
///
/// ## Example Usage
///
/// ### Accept cross-account request
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_endpoint_service::create(
///         "example",
///         VpcEndpointServiceArgs::builder()
///             .acceptance_required(false)
///             .network_load_balancer_arns(vec!["${exampleAwsLb.arn}",])
///             .build_struct(),
///     );
///     let exampleVpcEndpoint = vpc_endpoint::create(
///         "exampleVpcEndpoint",
///         VpcEndpointArgs::builder()
///             .private_dns_enabled(false)
///             .security_group_ids(vec!["${test.id}",])
///             .service_name("${testAwsVpcEndpointService.serviceName}")
///             .vpc_endpoint_type("Interface")
///             .vpc_id("${testAlternate.id}")
///             .build_struct(),
///     );
///     let exampleVpcEndpointConnectionAccepter = vpc_endpoint_connection_accepter::create(
///         "exampleVpcEndpointConnectionAccepter",
///         VpcEndpointConnectionAccepterArgs::builder()
///             .vpc_endpoint_id("${exampleVpcEndpoint.id}")
///             .vpc_endpoint_service_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Services using ID of the connection, which is the `VPC Endpoint Service ID` and `VPC Endpoint ID` separated by underscore (`_`).. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointConnectionAccepter:VpcEndpointConnectionAccepter foo vpce-svc-0f97a19d3fa8220bc_vpce-010601a6db371e263
/// ```
pub mod vpc_endpoint_connection_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionAccepterArgs {
        /// AWS VPC Endpoint ID.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// AWS VPC Endpoint Service ID.
        #[builder(into)]
        pub vpc_endpoint_service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointConnectionAccepterResult {
        /// AWS VPC Endpoint ID.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// AWS VPC Endpoint Service ID.
        pub vpc_endpoint_service_id: pulumi_wasm_rust::Output<String>,
        /// State of the VPC Endpoint.
        pub vpc_endpoint_state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcEndpointConnectionAccepterArgs,
    ) -> VpcEndpointConnectionAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let vpc_endpoint_service_id_binding = args.vpc_endpoint_service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointConnectionAccepter:VpcEndpointConnectionAccepter"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointServiceId".into(),
                    value: &vpc_endpoint_service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "vpcEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointServiceId".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointState".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointConnectionAccepterResult {
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
            vpc_endpoint_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointServiceId").unwrap(),
            ),
            vpc_endpoint_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointState").unwrap(),
            ),
        }
    }
}