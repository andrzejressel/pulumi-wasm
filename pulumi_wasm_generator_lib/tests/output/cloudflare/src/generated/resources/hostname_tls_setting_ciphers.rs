#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HostnameTlsSettingCiphersArgs {
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Ports to use within the IP rule.
    #[builder(into, default)]
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    /// Ciphers suites value.
    #[builder(into)]
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct HostnameTlsSettingCiphersResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Ports to use within the IP rule.
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    /// Ciphers suites value.
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: HostnameTlsSettingCiphersArgs,
) -> HostnameTlsSettingCiphersResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let hostname_binding = args.hostname.get_inner();
    let ports_binding = args.ports.get_inner();
    let values_binding = args.values.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "hostname".into(),
                value: &hostname_binding,
            },
            register_interface::ObjectField {
                name: "ports".into(),
                value: &ports_binding,
            },
            register_interface::ObjectField {
                name: "values".into(),
                value: &values_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "createdAt".into() },
            register_interface::ResultField { name : "hostname".into() },
            register_interface::ResultField { name : "ports".into() },
            register_interface::ResultField { name : "updatedAt".into() },
            register_interface::ResultField { name : "values".into() },
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
    HostnameTlsSettingCiphersResult {
        created_at: into_domain(hashmap.remove("createdAt").unwrap()),
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        ports: into_domain(hashmap.remove("ports").unwrap()),
        updated_at: into_domain(hashmap.remove("updatedAt").unwrap()),
        values: into_domain(hashmap.remove("values").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
