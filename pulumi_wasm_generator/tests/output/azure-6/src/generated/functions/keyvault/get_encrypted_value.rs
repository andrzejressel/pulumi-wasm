pub mod get_encrypted_value {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEncryptedValueArgs {
        /// The Algorithm which should be used to Decrypt/Encrypt this Value. Possible values are `RSA1_5`, `RSA-OAEP` and `RSA-OAEP-256`.
        #[builder(into)]
        pub algorithm: pulumi_wasm_rust::Output<String>,
        /// The Base64 URL Encoded Encrypted Data which should be decrypted into `plain_text_value`.
        #[builder(into, default)]
        pub encrypted_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Key Vault Key which should be used to Decrypt/Encrypt this Value.
        #[builder(into)]
        pub key_vault_key_id: pulumi_wasm_rust::Output<String>,
        /// The plain-text value which should be Encrypted into `encrypted_data`.
        ///
        /// > **Note:** One of either `encrypted_data` or `plain_text_value` must be specified and is used to populate the encrypted/decrypted value for the other field.
        #[builder(into, default)]
        pub plain_text_value: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEncryptedValueResult {
        pub algorithm: pulumi_wasm_rust::Output<String>,
        /// The Base64URL decoded string of `plain_text_value`. Because the API would remove padding characters of `plain_text_value` when encrypting, this attribute is useful to get the original value.
        pub decoded_plain_text_value: pulumi_wasm_rust::Output<String>,
        pub encrypted_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key_vault_key_id: pulumi_wasm_rust::Output<String>,
        pub plain_text_value: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEncryptedValueArgs) -> GetEncryptedValueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let algorithm_binding = args.algorithm.get_inner();
        let encrypted_data_binding = args.encrypted_data.get_inner();
        let key_vault_key_id_binding = args.key_vault_key_id.get_inner();
        let plain_text_value_binding = args.plain_text_value.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getEncryptedValue:getEncryptedValue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "algorithm".into(),
                    value: &algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "encryptedData".into(),
                    value: &encrypted_data_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "plainTextValue".into(),
                    value: &plain_text_value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "algorithm".into(),
                },
                register_interface::ResultField {
                    name: "decodedPlainTextValue".into(),
                },
                register_interface::ResultField {
                    name: "encryptedData".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultKeyId".into(),
                },
                register_interface::ResultField {
                    name: "plainTextValue".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEncryptedValueResult {
            algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("algorithm").unwrap(),
            ),
            decoded_plain_text_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("decodedPlainTextValue").unwrap(),
            ),
            encrypted_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedData").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultKeyId").unwrap(),
            ),
            plain_text_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plainTextValue").unwrap(),
            ),
        }
    }
}
