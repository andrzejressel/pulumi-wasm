pub mod get_secrets {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretsArgs {
        /// One or more encrypted payload definitions from the KMS service. See the Secret Definitions below.
        #[builder(into)]
        pub secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetSecretsSecret>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSecretsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Map containing each `secret` `name` as the key with its decrypted plaintext value
        pub plaintext: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub secrets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetSecretsSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSecretsArgs) -> GetSecretsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let secrets_binding = args.secrets.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getSecrets:getSecrets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "plaintext".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSecretsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            plaintext: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plaintext").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
        }
    }
}
