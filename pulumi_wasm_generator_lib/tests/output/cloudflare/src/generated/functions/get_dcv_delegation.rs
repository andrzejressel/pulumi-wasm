#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetDcvDelegationArgs {
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct GetDcvDelegationResult {
    /// The DCV Delegation hostname
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The DCV Delegation unique identifier
    pub id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetDcvDelegationArgs) -> GetDcvDelegationResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getDcvDelegation:getDcvDelegation".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "hostname".into() },
            register_interface::ResultField { name : "id".into() },
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
    GetDcvDelegationResult {
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
