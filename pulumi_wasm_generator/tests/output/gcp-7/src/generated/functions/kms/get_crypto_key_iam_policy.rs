pub mod get_crypto_key_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeyIamPolicyArgs {
        /// The crypto key ID, in the form
        #[builder(into)]
        pub crypto_key_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeyIamPolicyResult {
        pub crypto_key_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCryptoKeyIamPolicyArgs) -> GetCryptoKeyIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_key_id_binding = args.crypto_key_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getCryptoKeyIamPolicy:getCryptoKeyIamPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKeyId".into(),
                    value: &crypto_key_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cryptoKeyId".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCryptoKeyIamPolicyResult {
            crypto_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKeyId").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
        }
    }
}
