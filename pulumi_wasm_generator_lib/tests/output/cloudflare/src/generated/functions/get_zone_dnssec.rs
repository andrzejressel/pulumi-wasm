#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneDnssecArgs {
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct GetZoneDnssecResult {
    /// Zone DNSSEC algorithm.
    pub algorithm: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC digest.
    pub digest: pulumi_wasm_rust::Output<String>,
    /// Digest algorithm use for Zone DNSSEC.
    pub digest_algorithm: pulumi_wasm_rust::Output<String>,
    /// Digest Type for Zone DNSSEC.
    pub digest_type: pulumi_wasm_rust::Output<String>,
    /// DS for the Zone DNSSEC.
    pub ds: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC flags.
    pub flags: pulumi_wasm_rust::Output<i32>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Key Tag for the Zone DNSSEC.
    pub key_tag: pulumi_wasm_rust::Output<i32>,
    /// Key type used for Zone DNSSEC.
    pub key_type: pulumi_wasm_rust::Output<String>,
    /// Public Key for the Zone DNSSEC.
    pub public_key: pulumi_wasm_rust::Output<String>,
    /// The status of the Zone DNSSEC.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetZoneDnssecArgs) -> GetZoneDnssecResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getZoneDnssec:getZoneDnssec".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "algorithm".into() },
            register_interface::ResultField { name : "digest".into() },
            register_interface::ResultField { name : "digestAlgorithm".into() },
            register_interface::ResultField { name : "digestType".into() },
            register_interface::ResultField { name : "ds".into() },
            register_interface::ResultField { name : "flags".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "keyTag".into() },
            register_interface::ResultField { name : "keyType".into() },
            register_interface::ResultField { name : "publicKey".into() },
            register_interface::ResultField { name : "status".into() },
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
    GetZoneDnssecResult {
        algorithm: into_domain(hashmap.remove("algorithm").unwrap()),
        digest: into_domain(hashmap.remove("digest").unwrap()),
        digest_algorithm: into_domain(hashmap.remove("digestAlgorithm").unwrap()),
        digest_type: into_domain(hashmap.remove("digestType").unwrap()),
        ds: into_domain(hashmap.remove("ds").unwrap()),
        flags: into_domain(hashmap.remove("flags").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        key_tag: into_domain(hashmap.remove("keyTag").unwrap()),
        key_type: into_domain(hashmap.remove("keyType").unwrap()),
        public_key: into_domain(hashmap.remove("publicKey").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
