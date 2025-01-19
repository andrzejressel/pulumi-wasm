/// Provides a Route 53 Resolver DNSSEC config resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc::create(
///         "example",
///         VpcArgs::builder()
///             .cidr_block("10.0.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let exampleResolverDnsSecConfig = resolver_dns_sec_config::create(
///         "exampleResolverDnsSecConfig",
///         ResolverDnsSecConfigArgs::builder().resource_id("${example.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import  Route 53 Resolver DNSSEC configs using the Route 53 Resolver DNSSEC config ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverDnsSecConfig:ResolverDnsSecConfig example rdsc-be1866ecc1683e95
/// ```
pub mod resolver_dns_sec_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverDnsSecConfigArgs {
        /// The ID of the virtual private cloud (VPC) that you're updating the DNSSEC validation status for.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverDnsSecConfigResult {
        /// The ARN for a configuration for DNSSEC validation.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The owner account ID of the virtual private cloud (VPC) for a configuration for DNSSEC validation.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual private cloud (VPC) that you're updating the DNSSEC validation status for.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The validation status for a DNSSEC configuration. The status can be one of the following: `ENABLING`, `ENABLED`, `DISABLING` and `DISABLED`.
        pub validation_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResolverDnsSecConfigArgs,
    ) -> ResolverDnsSecConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverDnsSecConfig:ResolverDnsSecConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "validationStatus".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResolverDnsSecConfigResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            validation_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationStatus").unwrap(),
            ),
        }
    }
}
