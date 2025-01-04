pub mod get_crypto_key_latest_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeyLatestVersionArgs {
        /// The `id` of the Google Cloud Platform CryptoKey to which the key version belongs. This is also the `id` field of the
        /// `gcp.kms.CryptoKey` resource/datasource.
        #[builder(into)]
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        /// The filter argument is used to add a filter query parameter that limits which type of cryptoKeyVersion is retrieved as the latest by the data source: ?filter={{filter}}. When no value is provided there is no filtering.
        ///
        /// Example filter values if filtering on state.
        ///
        /// * `"state:ENABLED"` will retrieve the latest cryptoKeyVersion that has the state "ENABLED".
        ///
        /// [See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeyLatestVersionResult {
        /// The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
        pub algorithm: pulumi_wasm_rust::Output<String>,
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion. See the [protection_level reference](https://cloud.google.com/kms/docs/reference/rest/v1/ProtectionLevel) for possible outputs.
        pub protection_level: pulumi_wasm_rust::Output<String>,
        /// If the enclosing CryptoKey has purpose `ASYMMETRIC_SIGN` or `ASYMMETRIC_DECRYPT`, this block contains details about the public key associated to this CryptoKeyVersion. Structure is documented below.
        pub public_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeyLatestVersionPublicKey>,
        >,
        /// The current state of the latest CryptoKeyVersion. See the [state reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions#CryptoKeyVersion.CryptoKeyVersionState) for possible outputs.
        pub state: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetCryptoKeyLatestVersionArgs,
    ) -> GetCryptoKeyLatestVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_key_binding = args.crypto_key.get_inner();
        let filter_binding = args.filter.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getCryptoKeyLatestVersion:getCryptoKeyLatestVersion".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKey".into(),
                    value: &crypto_key_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "algorithm".into(),
                },
                register_interface::ResultField {
                    name: "cryptoKey".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "protectionLevel".into(),
                },
                register_interface::ResultField {
                    name: "publicKeys".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCryptoKeyLatestVersionResult {
            algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("algorithm").unwrap(),
            ),
            crypto_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKey").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            protection_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectionLevel").unwrap(),
            ),
            public_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeys").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
