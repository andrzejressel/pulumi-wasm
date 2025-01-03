pub mod get_kms_crypto_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyArgs {
        /// The `id` of the Google Cloud Platform KeyRing to which the key belongs.
        #[builder(into)]
        pub key_ring: pulumi_wasm_rust::Output<String>,
        /// The CryptoKey's name.
        /// A CryptoKey’s name belonging to the specified Google Cloud Platform KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyResult {
        pub crypto_key_backend: pulumi_wasm_rust::Output<String>,
        pub destroy_scheduled_duration: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub import_only: pulumi_wasm_rust::Output<bool>,
        pub key_access_justifications_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::kms::GetKmsCryptoKeyKeyAccessJustificationsPolicy,
            >,
        >,
        pub key_ring: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub primaries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetKmsCryptoKeyPrimary>,
        >,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines the cryptographic capabilities of the key.
        pub purpose: pulumi_wasm_rust::Output<String>,
        /// Every time this period passes, generate a new CryptoKeyVersion and set it as
        /// the primary. The first rotation will take place after the specified period. The rotation period has the format
        /// of a decimal number with up to 9 fractional digits, followed by the letter s (seconds).
        pub rotation_period: pulumi_wasm_rust::Output<String>,
        pub skip_initial_version_creation: pulumi_wasm_rust::Output<bool>,
        pub version_templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kms::GetKmsCryptoKeyVersionTemplate>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKmsCryptoKeyArgs) -> GetKmsCryptoKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_ring_binding = args.key_ring.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getKMSCryptoKey:getKMSCryptoKey".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyRing".into(),
                    value: &key_ring_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cryptoKeyBackend".into(),
                },
                register_interface::ResultField {
                    name: "destroyScheduledDuration".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "importOnly".into(),
                },
                register_interface::ResultField {
                    name: "keyAccessJustificationsPolicies".into(),
                },
                register_interface::ResultField {
                    name: "keyRing".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaries".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "purpose".into(),
                },
                register_interface::ResultField {
                    name: "rotationPeriod".into(),
                },
                register_interface::ResultField {
                    name: "skipInitialVersionCreation".into(),
                },
                register_interface::ResultField {
                    name: "versionTemplates".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKmsCryptoKeyResult {
            crypto_key_backend: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKeyBackend").unwrap(),
            ),
            destroy_scheduled_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destroyScheduledDuration").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            import_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importOnly").unwrap(),
            ),
            key_access_justifications_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyAccessJustificationsPolicies").unwrap(),
            ),
            key_ring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRing").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaries").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            purpose: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purpose").unwrap(),
            ),
            rotation_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rotationPeriod").unwrap(),
            ),
            skip_initial_version_creation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipInitialVersionCreation").unwrap(),
            ),
            version_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionTemplates").unwrap(),
            ),
        }
    }
}
