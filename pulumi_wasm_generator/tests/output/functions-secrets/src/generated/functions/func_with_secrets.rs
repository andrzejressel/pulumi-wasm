pub mod func_with_secrets {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithSecretsArgs {
        #[builder(into)]
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub plaintext: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FuncWithSecretsResult {
        pub ciphertext: pulumi_wasm_rust::Output<String>,
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub plaintext: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: FuncWithSecretsArgs) -> FuncWithSecretsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "ciphertext".into(),
                },
                register_interface::ResultField {
                    name: "cryptoKey".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "plaintext".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FuncWithSecretsResult {
            ciphertext: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ciphertext").unwrap(),
            ),
            crypto_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKey").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            plaintext: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plaintext").unwrap(),
            ),
        }
    }
}
