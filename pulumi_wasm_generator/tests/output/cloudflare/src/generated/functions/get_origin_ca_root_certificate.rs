#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetOriginCaRootCertificateArgs {
    /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
    #[builder(into)]
    pub algorithm: pulumi_wasm_rust::Output<String>,
}
pub struct GetOriginCaRootCertificateResult {
    /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
    pub algorithm: pulumi_wasm_rust::Output<String>,
    /// The Origin CA root certificate in PEM format.
    pub cert_pem: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetOriginCaRootCertificateArgs) -> GetOriginCaRootCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let algorithm_binding = args.algorithm.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getOriginCaRootCertificate:getOriginCaRootCertificate"
            .into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "algorithm".into(),
                value: &algorithm_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "algorithm".into() },
            register_interface::ResultField { name : "certPem".into() },
            register_interface::ResultField { name : "id".into() },
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
    GetOriginCaRootCertificateResult {
        algorithm: into_domain(hashmap.remove("algorithm").unwrap()),
        cert_pem: into_domain(hashmap.remove("certPem").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
    }
}
