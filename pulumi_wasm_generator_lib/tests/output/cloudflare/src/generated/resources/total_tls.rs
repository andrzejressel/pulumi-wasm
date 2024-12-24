#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TotalTlsArgs {
    /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
    #[builder(into, default)]
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable Total TLS for the zone.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct TotalTlsResult {
    /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable Total TLS for the zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TotalTlsArgs) -> TotalTlsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let certificate_authority_binding = args.certificate_authority.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/totalTls:TotalTls".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "certificateAuthority".into(),
                value: &certificate_authority_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "certificateAuthority".into() },
            register_interface::ResultField { name : "enabled".into() },
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
    TotalTlsResult {
        certificate_authority: into_domain(
            hashmap.remove("certificateAuthority").unwrap(),
        ),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
