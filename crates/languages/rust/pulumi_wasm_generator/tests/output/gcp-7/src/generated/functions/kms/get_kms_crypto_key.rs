pub mod get_kms_crypto_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyArgs {
        /// The `id` of the Google Cloud Platform KeyRing to which the key belongs.
        #[builder(into)]
        pub key_ring: pulumi_wasm_rust::InputOrOutput<String>,
        /// The CryptoKey's name.
        /// A CryptoKey’s name belonging to the specified Google Cloud Platform KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKmsCryptoKeyArgs,
    ) -> GetKmsCryptoKeyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_ring_binding = args.key_ring.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getKMSCryptoKey:getKMSCryptoKey".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKmsCryptoKeyResult {
            crypto_key_backend: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cryptoKeyBackend"),
            ),
            destroy_scheduled_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destroyScheduledDuration"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            import_only: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("importOnly"),
            ),
            key_access_justifications_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyAccessJustificationsPolicies"),
            ),
            key_ring: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyRing"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primaries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaries"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            purpose: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("purpose"),
            ),
            rotation_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rotationPeriod"),
            ),
            skip_initial_version_creation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipInitialVersionCreation"),
            ),
            version_templates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionTemplates"),
            ),
        }
    }
}
