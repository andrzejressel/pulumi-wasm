#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideArgs {
    #[builder(into, default)]
    pub settings: pulumi_wasm_rust::Output<
        Option<super::types::ZoneSettingsOverrideSettings>,
    >,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ZoneSettingsOverrideResult {
    pub initial_settings: pulumi_wasm_rust::Output<
        Vec<super::types::ZoneSettingsOverrideInitialSetting>,
    >,
    pub initial_settings_read_at: pulumi_wasm_rust::Output<String>,
    pub readonly_settings: pulumi_wasm_rust::Output<Vec<String>>,
    pub settings: pulumi_wasm_rust::Output<super::types::ZoneSettingsOverrideSettings>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
    pub zone_status: pulumi_wasm_rust::Output<String>,
    pub zone_type: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneSettingsOverrideArgs) -> ZoneSettingsOverrideResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let settings_binding = args.settings.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zoneSettingsOverride:ZoneSettingsOverride".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "settings".into(),
                value: &settings_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "initialSettings".into() },
            register_interface::ResultField { name : "initialSettingsReadAt".into() },
            register_interface::ResultField { name : "readonlySettings".into() },
            register_interface::ResultField { name : "settings".into() },
            register_interface::ResultField { name : "zoneId".into() },
            register_interface::ResultField { name : "zoneStatus".into() },
            register_interface::ResultField { name : "zoneType".into() },
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
    ZoneSettingsOverrideResult {
        initial_settings: into_domain(hashmap.remove("initialSettings").unwrap()),
        initial_settings_read_at: into_domain(
            hashmap.remove("initialSettingsReadAt").unwrap(),
        ),
        readonly_settings: into_domain(hashmap.remove("readonlySettings").unwrap()),
        settings: into_domain(hashmap.remove("settings").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
        zone_status: into_domain(hashmap.remove("zoneStatus").unwrap()),
        zone_type: into_domain(hashmap.remove("zoneType").unwrap()),
    }
}
