/// Provides a Route 53 Resolver DNS Firewall config resource.
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
///     let exampleResolverFirewallConfig = resolver_firewall_config::create(
///         "exampleResolverFirewallConfig",
///         ResolverFirewallConfigArgs::builder()
///             .firewall_fail_open("ENABLED")
///             .resource_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Resolver DNS Firewall configs using the Route 53 Resolver DNS Firewall config ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverFirewallConfig:ResolverFirewallConfig example rdsc-be1866ecc1683e95
/// ```
pub mod resolver_firewall_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverFirewallConfigArgs {
        /// Determines how Route 53 Resolver handles queries during failures, for example when all traffic that is sent to DNS Firewall fails to receive a reply. By default, fail open is disabled, which means the failure mode is closed. This approach favors security over availability. DNS Firewall blocks queries that it is unable to evaluate properly. If you enable this option, the failure mode is open. This approach favors availability over security. DNS Firewall allows queries to proceed if it is unable to properly evaluate them. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub firewall_fail_open: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the VPC that the configuration is for.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverFirewallConfigResult {
        /// Determines how Route 53 Resolver handles queries during failures, for example when all traffic that is sent to DNS Firewall fails to receive a reply. By default, fail open is disabled, which means the failure mode is closed. This approach favors security over availability. DNS Firewall blocks queries that it is unable to evaluate properly. If you enable this option, the failure mode is open. This approach favors availability over security. DNS Firewall allows queries to proceed if it is unable to properly evaluate them. Valid values: `ENABLED`, `DISABLED`.
        pub firewall_fail_open: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID of the owner of the VPC that this firewall configuration applies to.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC that the configuration is for.
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResolverFirewallConfigArgs,
    ) -> ResolverFirewallConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let firewall_fail_open_binding = args.firewall_fail_open.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverFirewallConfig:ResolverFirewallConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "firewallFailOpen".into(),
                    value: &firewall_fail_open_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "firewallFailOpen".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResolverFirewallConfigResult {
            firewall_fail_open: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallFailOpen").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}