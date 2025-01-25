/// Provides a resource, that manages GRE tunnels for Magic Transit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = magic_wan_gre_tunnel::create(
///         "example",
///         MagicWanGreTunnelArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .cloudflare_gre_endpoint("203.0.113.2")
///             .customer_gre_endpoint("203.0.113.1")
///             .description("Tunnel for ISP X")
///             .health_check_enabled(true)
///             .health_check_target("203.0.113.1")
///             .health_check_type("reply")
///             .interface_address("192.0.2.0/31")
///             .mtu(1476)
///             .name("GRE_1")
///             .ttl(64)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/magicWanGreTunnel:MagicWanGreTunnel example <account_id>/<tunnel_id>
/// ```
///
pub mod magic_wan_gre_tunnel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MagicWanGreTunnelArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IP address assigned to the Cloudflare side of the GRE tunnel.
        #[builder(into)]
        pub cloudflare_gre_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// The IP address assigned to the customer side of the GRE tunnel.
        #[builder(into)]
        pub customer_gre_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the GRE tunnel intent.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies if ICMP tunnel health checks are enabled.
        #[builder(into, default)]
        pub health_check_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The IP address of the customer endpoint that will receive tunnel health checks.
        #[builder(into, default)]
        pub health_check_target: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
        #[builder(into, default)]
        pub health_check_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
        #[builder(into)]
        pub interface_address: pulumi_wasm_rust::InputOrOutput<String>,
        /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
        #[builder(into, default)]
        pub mtu: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Name of the GRE tunnel.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Time To Live (TTL) in number of hops of the GRE tunnel.
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct MagicWanGreTunnelResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP address assigned to the Cloudflare side of the GRE tunnel.
        pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
        /// The IP address assigned to the customer side of the GRE tunnel.
        pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
        /// Description of the GRE tunnel intent.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if ICMP tunnel health checks are enabled.
        pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
        /// The IP address of the customer endpoint that will receive tunnel health checks.
        pub health_check_target: pulumi_wasm_rust::Output<String>,
        /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
        pub health_check_type: pulumi_wasm_rust::Output<String>,
        /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
        pub interface_address: pulumi_wasm_rust::Output<String>,
        /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
        pub mtu: pulumi_wasm_rust::Output<i32>,
        /// Name of the GRE tunnel.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Time To Live (TTL) in number of hops of the GRE tunnel.
        pub ttl: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MagicWanGreTunnelArgs,
    ) -> MagicWanGreTunnelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let cloudflare_gre_endpoint_binding = args
            .cloudflare_gre_endpoint
            .get_output(context)
            .get_inner();
        let customer_gre_endpoint_binding = args
            .customer_gre_endpoint
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let health_check_enabled_binding = args
            .health_check_enabled
            .get_output(context)
            .get_inner();
        let health_check_target_binding = args
            .health_check_target
            .get_output(context)
            .get_inner();
        let health_check_type_binding = args
            .health_check_type
            .get_output(context)
            .get_inner();
        let interface_address_binding = args
            .interface_address
            .get_output(context)
            .get_inner();
        let mtu_binding = args.mtu.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/magicWanGreTunnel:MagicWanGreTunnel".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "cloudflareGreEndpoint".into(),
                    value: &cloudflare_gre_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "customerGreEndpoint".into(),
                    value: &customer_gre_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckEnabled".into(),
                    value: &health_check_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckTarget".into(),
                    value: &health_check_target_binding,
                },
                register_interface::ObjectField {
                    name: "healthCheckType".into(),
                    value: &health_check_type_binding,
                },
                register_interface::ObjectField {
                    name: "interfaceAddress".into(),
                    value: &interface_address_binding,
                },
                register_interface::ObjectField {
                    name: "mtu".into(),
                    value: &mtu_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "cloudflareGreEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "customerGreEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckEnabled".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckTarget".into(),
                },
                register_interface::ResultField {
                    name: "healthCheckType".into(),
                },
                register_interface::ResultField {
                    name: "interfaceAddress".into(),
                },
                register_interface::ResultField {
                    name: "mtu".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MagicWanGreTunnelResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            cloudflare_gre_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudflareGreEndpoint").unwrap(),
            ),
            customer_gre_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerGreEndpoint").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            health_check_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckEnabled").unwrap(),
            ),
            health_check_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckTarget").unwrap(),
            ),
            health_check_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthCheckType").unwrap(),
            ),
            interface_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interfaceAddress").unwrap(),
            ),
            mtu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mtu").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ttl").unwrap()),
        }
    }
}
