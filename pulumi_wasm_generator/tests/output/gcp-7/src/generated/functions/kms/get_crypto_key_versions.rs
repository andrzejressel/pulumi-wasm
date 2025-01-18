pub mod get_crypto_key_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeyVersionsArgs {
        /// The `id` of the Google Cloud Platform CryptoKey to which the key version belongs. This is also the `id` field of the
        /// `gcp.kms.CryptoKey` resource/datasource.
        #[builder(into)]
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        /// The filter argument is used to add a filter query parameter that limits which versions are retrieved by the data source: ?filter={{filter}}. When no value is provided there is no filtering.
        ///
        /// Example filter values if filtering on name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}/cryptoKeyVersions.
        ///
        /// * `"name:my-key-"` will retrieve cryptoKeyVersions that contain "my-key-" anywhere in their name.
        /// * `"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1/cryptoKeyVersions/my-version-1"` will only retrieve a key with that exact name.
        ///
        /// [See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeyVersionsResult {
        pub crypto_key: pulumi_wasm_rust::Output<String>,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub public_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeyVersionsPublicKey>,
        >,
        /// A list of all the retrieved crypto key versions from the provided crypto key. This list is influenced by the provided filter argument.
        pub versions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeyVersionsVersion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCryptoKeyVersionsArgs) -> GetCryptoKeyVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_key_binding = args.crypto_key.get_inner();
        let filter_binding = args.filter.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getCryptoKeyVersions:getCryptoKeyVersions".into(),
            version: super::super::super::get_version(),
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
                    name: "cryptoKey".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "publicKeys".into(),
                },
                register_interface::ResultField {
                    name: "versions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCryptoKeyVersionsResult {
            crypto_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKey").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            public_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeys").unwrap(),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versions").unwrap(),
            ),
        }
    }
}
