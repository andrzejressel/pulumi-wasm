#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct MagicWanGreTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The IP address assigned to the Cloudflare side of the GRE tunnel.
    #[builder(into)]
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// The IP address assigned to the customer side of the GRE tunnel.
    #[builder(into)]
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// Description of the GRE tunnel intent.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if ICMP tunnel health checks are enabled.
    #[builder(into, default)]
    pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The IP address of the customer endpoint that will receive tunnel health checks.
    #[builder(into, default)]
    pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
    #[builder(into, default)]
    pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    #[builder(into)]
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
    #[builder(into, default)]
    pub mtu: pulumi_wasm_rust::Output<Option<i32>>,
    /// Name of the GRE tunnel.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Time To Live (TTL) in number of hops of the GRE tunnel.
    #[builder(into, default)]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
}
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
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: MagicWanGreTunnelArgs) -> MagicWanGreTunnelResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let cloudflare_gre_endpoint_binding = args.cloudflare_gre_endpoint.get_inner();
    let customer_gre_endpoint_binding = args.customer_gre_endpoint.get_inner();
    let description_binding = args.description.get_inner();
    let health_check_enabled_binding = args.health_check_enabled.get_inner();
    let health_check_target_binding = args.health_check_target.get_inner();
    let health_check_type_binding = args.health_check_type.get_inner();
    let interface_address_binding = args.interface_address.get_inner();
    let mtu_binding = args.mtu.get_inner();
    let name_binding = args.name.get_inner();
    let ttl_binding = args.ttl.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/magicWanGreTunnel:MagicWanGreTunnel".into(),
        name: name.to_string(),
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
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "cloudflareGreEndpoint".into() },
            register_interface::ResultField { name : "customerGreEndpoint".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "healthCheckEnabled".into() },
            register_interface::ResultField { name : "healthCheckTarget".into() },
            register_interface::ResultField { name : "healthCheckType".into() },
            register_interface::ResultField { name : "interfaceAddress".into() },
            register_interface::ResultField { name : "mtu".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "ttl".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    MagicWanGreTunnelResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        cloudflare_gre_endpoint: into_domain(
            hashmap.remove("cloudflareGreEndpoint").unwrap(),
        ),
        customer_gre_endpoint: into_domain(
            hashmap.remove("customerGreEndpoint").unwrap(),
        ),
        description: into_domain(hashmap.remove("description").unwrap()),
        health_check_enabled: into_domain(hashmap.remove("healthCheckEnabled").unwrap()),
        health_check_target: into_domain(hashmap.remove("healthCheckTarget").unwrap()),
        health_check_type: into_domain(hashmap.remove("healthCheckType").unwrap()),
        interface_address: into_domain(hashmap.remove("interfaceAddress").unwrap()),
        mtu: into_domain(hashmap.remove("mtu").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        ttl: into_domain(hashmap.remove("ttl").unwrap()),
    }
}
