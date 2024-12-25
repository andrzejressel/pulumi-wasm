#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithSecretsArgs {
    #[builder(into)]
    pub crypto_key: pulumi_wasm_rust::Output<String>,
    #[builder(into)]
    pub plaintext: pulumi_wasm_rust::Output<String>,
}
pub struct FuncWithSecretsResult {
    pub ciphertext: pulumi_wasm_rust::Output<String>,
    pub crypto_key: pulumi_wasm_rust::Output<String>,
    pub id: pulumi_wasm_rust::Output<String>,
    pub plaintext: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: FuncWithSecretsArgs) -> FuncWithSecretsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let crypto_key_binding = args.crypto_key.get_inner();
    let plaintext_binding = args.plaintext.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "mypkg::funcWithSecrets".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "cryptoKey".into(),
                value: &crypto_key_binding,
            },
            register_interface::ObjectField {
                name: "plaintext".into(),
                value: &plaintext_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "ciphertext".into() },
            register_interface::ResultField { name : "cryptoKey".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "plaintext".into() },
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
    FuncWithSecretsResult {
        ciphertext: into_domain(hashmap.remove("ciphertext").unwrap()),
        crypto_key: into_domain(hashmap.remove("cryptoKey").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        plaintext: into_domain(hashmap.remove("plaintext").unwrap()),
    }
}
