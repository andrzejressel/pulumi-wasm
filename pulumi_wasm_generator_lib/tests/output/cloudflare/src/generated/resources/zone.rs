#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneArgs {
    /// Account ID to manage the zone resource in.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to scan for DNS records on creation. Ignored after zone is created.
    #[builder(into, default)]
    pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`.
    #[builder(into, default)]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`.
    #[builder(into, default)]
    pub plan: pulumi_wasm_rust::Output<Option<String>>,
    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`, `secondary`. Defaults to `full`.
    #[builder(into, default)]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Vanity Nameservers (if set).
    #[builder(into, default)]
    pub vanity_name_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone: pulumi_wasm_rust::Output<String>,
}
pub struct ZoneResult {
    /// Account ID to manage the zone resource in.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to scan for DNS records on creation. Ignored after zone is created.
    pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
    pub meta: pulumi_wasm_rust::Output<std::collections::HashMap<String, bool>>,
    /// Cloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS.
    pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Whether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`.
    pub plan: pulumi_wasm_rust::Output<String>,
    /// Status of the zone. Available values: `active`, `pending`, `initializing`, `moved`, `deleted`, `deactivated`.
    pub status: pulumi_wasm_rust::Output<String>,
    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`, `secondary`. Defaults to `full`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Vanity Nameservers (if set).
    pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Contains the TXT record value to validate domain ownership. This is only populated for zones of type `partial`.
    pub verification_key: pulumi_wasm_rust::Output<String>,
    /// The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**
    pub zone: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneArgs) -> ZoneResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let jump_start_binding = args.jump_start.get_inner();
    let paused_binding = args.paused.get_inner();
    let plan_binding = args.plan.get_inner();
    let type__binding = args.type_.get_inner();
    let vanity_name_servers_binding = args.vanity_name_servers.get_inner();
    let zone_binding = args.zone.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zone:Zone".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "jumpStart".into(),
                value: &jump_start_binding,
            },
            register_interface::ObjectField {
                name: "paused".into(),
                value: &paused_binding,
            },
            register_interface::ObjectField {
                name: "plan".into(),
                value: &plan_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "vanityNameServers".into(),
                value: &vanity_name_servers_binding,
            },
            register_interface::ObjectField {
                name: "zone".into(),
                value: &zone_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "jumpStart".into() },
            register_interface::ResultField { name : "meta".into() },
            register_interface::ResultField { name : "nameServers".into() },
            register_interface::ResultField { name : "paused".into() },
            register_interface::ResultField { name : "plan".into() },
            register_interface::ResultField { name : "status".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "vanityNameServers".into() },
            register_interface::ResultField { name : "verificationKey".into() },
            register_interface::ResultField { name : "zone".into() },
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
    ZoneResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        jump_start: into_domain(hashmap.remove("jumpStart").unwrap()),
        meta: into_domain(hashmap.remove("meta").unwrap()),
        name_servers: into_domain(hashmap.remove("nameServers").unwrap()),
        paused: into_domain(hashmap.remove("paused").unwrap()),
        plan: into_domain(hashmap.remove("plan").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        vanity_name_servers: into_domain(hashmap.remove("vanityNameServers").unwrap()),
        verification_key: into_domain(hashmap.remove("verificationKey").unwrap()),
        zone: into_domain(hashmap.remove("zone").unwrap()),
    }
}
