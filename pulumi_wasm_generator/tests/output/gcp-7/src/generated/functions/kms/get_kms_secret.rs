pub mod get_kms_secret {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsSecretArgs {
        /// The [additional authenticated data](https://cloud.google.com/kms/docs/additional-authenticated-data) used for integrity checks during encryption and decryption.
        #[builder(into, default)]
        pub additional_authenticated_data: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ciphertext to be decrypted, encoded in base64
        #[builder(into)]
        pub ciphertext: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the CryptoKey that will be used to
        /// decrypt the provided ciphertext. This is represented by the format
        /// `{projectId}/{location}/{keyRingName}/{cryptoKeyName}`.
        #[builder(into)]
        pub crypto_key: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsSecretResult {
        pub additional_authenticated_data: pulumi_wasm_rust::Output<Option<String>>,
        pub ciphertext: pulumi_wasm_rust::Output<String>,
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Contains the result of decrypting the provided ciphertext.
        pub plaintext: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKmsSecretArgs,
    ) -> GetKmsSecretResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_authenticated_data_binding = args
            .additional_authenticated_data
            .get_output(context)
            .get_inner();
        let ciphertext_binding = args.ciphertext.get_output(context).get_inner();
        let crypto_key_binding = args.crypto_key.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getKMSSecret:getKMSSecret".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalAuthenticatedData".into(),
                    value: &additional_authenticated_data_binding,
                },
                register_interface::ObjectField {
                    name: "ciphertext".into(),
                    value: &ciphertext_binding,
                },
                register_interface::ObjectField {
                    name: "cryptoKey".into(),
                    value: &crypto_key_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalAuthenticatedData".into(),
                },
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKmsSecretResult {
            additional_authenticated_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalAuthenticatedData").unwrap(),
            ),
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
