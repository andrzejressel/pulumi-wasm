/// Resource for enabling private DNS on an AWS VPC (Virtual Private Cloud) Endpoint.
///
/// > When using this resource, the `private_dns_enabled` argument should be omitted on the parent `aws.ec2.VpcEndpoint` resource.
/// Setting the value both places can lead to unintended behavior and persistent differences.
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
///     let example = endpoint_private_dns::create(
///         "example",
///         EndpointPrivateDnsArgs::builder()
///             .private_dns_enabled(true)
///             .vpc_endpoint_id("${exampleAwsVpcEndpoint.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a VPC (Virtual Private Cloud) Endpoint Private DNS using the `vpc_endpoint_id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpc/endpointPrivateDns:EndpointPrivateDns example vpce-abcd-1234
/// ```
pub mod endpoint_private_dns {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointPrivateDnsArgs {
        /// Indicates whether a private hosted zone is associated with the VPC. Only applicable for `Interface` endpoints.
        #[builder(into)]
        pub private_dns_enabled: pulumi_wasm_rust::Output<bool>,
        /// VPC endpoint identifier.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointPrivateDnsResult {
        /// Indicates whether a private hosted zone is associated with the VPC. Only applicable for `Interface` endpoints.
        pub private_dns_enabled: pulumi_wasm_rust::Output<bool>,
        /// VPC endpoint identifier.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EndpointPrivateDnsArgs) -> EndpointPrivateDnsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let private_dns_enabled_binding = args.private_dns_enabled.get_inner();
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpc/endpointPrivateDns:EndpointPrivateDns".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "privateDnsEnabled".into(),
                    value: &private_dns_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "privateDnsEnabled".into(),
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
        EndpointPrivateDnsResult {
            private_dns_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsEnabled").unwrap(),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
        }
    }
}
