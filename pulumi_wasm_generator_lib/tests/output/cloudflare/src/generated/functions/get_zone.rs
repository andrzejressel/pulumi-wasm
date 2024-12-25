#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneArgs {
    /// The account identifier to target for the resource.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the zone. Must provide only one of `zone_id`, `name`.
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct GetZoneResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The name of the zone. Must provide only one of `zone_id`, `name`.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Cloudflare assigned name servers. This is only populated for zones that use Cloudflare DNS.
    pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Whether the zone is paused on Cloudflare.
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The name of the plan associated with the zone.
    pub plan: pulumi_wasm_rust::Output<String>,
    /// Status of the zone.
    pub status: pulumi_wasm_rust::Output<String>,
    /// List of Vanity Nameservers (if set).
    pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetZoneArgs) -> GetZoneResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getZone:getZone".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "nameServers".into() },
            register_interface::ResultField { name : "paused".into() },
            register_interface::ResultField { name : "plan".into() },
            register_interface::ResultField { name : "status".into() },
            register_interface::ResultField { name : "vanityNameServers".into() },
            register_interface::ResultField { name : "zoneId".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetZoneResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        name_servers: into_domain(hashmap.remove("nameServers").unwrap()),
        paused: into_domain(hashmap.remove("paused").unwrap()),
        plan: into_domain(hashmap.remove("plan").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        vanity_name_servers: into_domain(hashmap.remove("vanityNameServers").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
