#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingCatchAllArgs {
    /// List actions patterns.
    #[builder(into)]
    pub actions: pulumi_wasm_rust::Output<Vec<super::types::EmailRoutingCatchAllAction>>,
    /// Routing rule status.
    #[builder(into, default)]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    #[builder(into)]
    pub matchers: pulumi_wasm_rust::Output<
        Vec<super::types::EmailRoutingCatchAllMatcher>,
    >,
    /// Routing rule name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct EmailRoutingCatchAllResult {
    /// List actions patterns.
    pub actions: pulumi_wasm_rust::Output<Vec<super::types::EmailRoutingCatchAllAction>>,
    /// Routing rule status.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    pub matchers: pulumi_wasm_rust::Output<
        Vec<super::types::EmailRoutingCatchAllMatcher>,
    >,
    /// Routing rule name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Routing rule identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: EmailRoutingCatchAllArgs) -> EmailRoutingCatchAllResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let actions_binding = args.actions.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let matchers_binding = args.matchers.get_inner();
    let name_binding = args.name.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/emailRoutingCatchAll:EmailRoutingCatchAll".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "actions".into(),
                value: &actions_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "matchers".into(),
                value: &matchers_binding,
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
            register_interface::ResultField { name : "actions".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "matchers".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "tag".into() },
            register_interface::ResultField { name : "zoneId".into() },
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
    EmailRoutingCatchAllResult {
        actions: into_domain(hashmap.remove("actions").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        matchers: into_domain(hashmap.remove("matchers").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        tag: into_domain(hashmap.remove("tag").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}