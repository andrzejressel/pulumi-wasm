#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingSettingsArgs {
    /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Flag to check if the user skipped the configuration wizard.
    #[builder(into, default)]
    pub skip_wizard: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct EmailRoutingSettingsResult {
    /// The date and time the settings have been created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The date and time the settings have been modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// Domain of your zone.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Flag to check if the user skipped the configuration wizard.
    pub skip_wizard: pulumi_wasm_rust::Output<bool>,
    /// Show the state of your account, and the type or configuration error.
    pub status: pulumi_wasm_rust::Output<String>,
    /// Email Routing settings identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingSettingsArgs) -> EmailRoutingSettingsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let enabled_binding = args.enabled.get_inner();
    let skip_wizard_binding = args.skip_wizard.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/emailRoutingSettings:EmailRoutingSettings".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "skipWizard".into(),
                value: &skip_wizard_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "created".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "modified".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "skipWizard".into() },
            register_interface::ResultField { name : "status".into() },
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
    EmailRoutingSettingsResult {
        created: into_domain(hashmap.remove("created").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        modified: into_domain(hashmap.remove("modified").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        skip_wizard: into_domain(hashmap.remove("skipWizard").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        tag: into_domain(hashmap.remove("tag").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
