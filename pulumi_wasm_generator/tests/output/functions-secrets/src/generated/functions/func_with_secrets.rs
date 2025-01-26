pub mod func_with_secrets {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithSecretsArgs {
        #[builder(into)]
        pub crypto_key: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub plaintext: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: FuncWithSecretsArgs,
    ) -> FuncWithSecretsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_key_binding = args.crypto_key.get_output(context).get_inner();
        let plaintext_binding = args.plaintext.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithSecrets".into(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        FuncWithSecretsResult {
            ciphertext: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ciphertext"),
            ),
            crypto_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cryptoKey"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            plaintext: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("plaintext"),
            ),
        }
    }
}
