pub mod get_kms_secret_asymmetric {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsSecretAsymmetricArgs {
        /// The ciphertext to be decrypted, encoded in base64
        #[builder(into)]
        pub ciphertext: pulumi_wasm_rust::Output<String>,
        /// The crc32 checksum of the `ciphertext` in hexadecimal notation. If not specified, it will be computed.
        #[builder(into, default)]
        pub crc32: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the CryptoKey version that will be used to
        /// decrypt the provided ciphertext. This is represented by the format
        /// `projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}/cryptoKeyVersions/{version}`.
        #[builder(into)]
        pub crypto_key_version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsSecretAsymmetricResult {
        pub ciphertext: pulumi_wasm_rust::Output<String>,
        /// Contains the crc32 checksum of the provided ciphertext.
        pub crc32: pulumi_wasm_rust::Output<Option<String>>,
        pub crypto_key_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Contains the result of decrypting the provided ciphertext.
        pub plaintext: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKmsSecretAsymmetricArgs) -> GetKmsSecretAsymmetricResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ciphertext_binding = args.ciphertext.get_inner();
        let crc32_binding = args.crc32.get_inner();
        let crypto_key_version_binding = args.crypto_key_version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getKMSSecretAsymmetric:getKMSSecretAsymmetric".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ciphertext".into(),
                    value: &ciphertext_binding,
                },
                register_interface::ObjectField {
                    name: "crc32".into(),
                    value: &crc32_binding,
                },
                register_interface::ObjectField {
                    name: "cryptoKeyVersion".into(),
                    value: &crypto_key_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "ciphertext".into(),
                },
                register_interface::ResultField {
                    name: "crc32".into(),
                },
                register_interface::ResultField {
                    name: "cryptoKeyVersion".into(),
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
        GetKmsSecretAsymmetricResult {
            ciphertext: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ciphertext").unwrap(),
            ),
            crc32: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("crc32").unwrap(),
            ),
            crypto_key_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKeyVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            plaintext: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plaintext").unwrap(),
            ),
        }
    }
}
