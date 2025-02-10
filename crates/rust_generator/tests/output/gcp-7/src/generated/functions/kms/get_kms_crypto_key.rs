#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kms_crypto_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyArgs {
        /// The `id` of the Google Cloud Platform KeyRing to which the key belongs.
        #[builder(into)]
        pub key_ring: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CryptoKey's name.
        /// A CryptoKey’s name belonging to the specified Google Cloud Platform KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyResult {
        pub crypto_key_backend: pulumi_gestalt_rust::Output<String>,
        pub destroy_scheduled_duration: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub import_only: pulumi_gestalt_rust::Output<bool>,
        pub key_access_justifications_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::kms::GetKmsCryptoKeyKeyAccessJustificationsPolicy,
            >,
        >,
        pub key_ring: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub primaries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetKmsCryptoKeyPrimary>,
        >,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines the cryptographic capabilities of the key.
        pub purpose: pulumi_gestalt_rust::Output<String>,
        /// Every time this period passes, generate a new CryptoKeyVersion and set it as
        /// the primary. The first rotation will take place after the specified period. The rotation period has the format
        /// of a decimal number with up to 9 fractional digits, followed by the letter s (seconds).
        pub rotation_period: pulumi_gestalt_rust::Output<String>,
        pub skip_initial_version_creation: pulumi_gestalt_rust::Output<bool>,
        pub version_templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetKmsCryptoKeyVersionTemplate>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKmsCryptoKeyArgs,
    ) -> GetKmsCryptoKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_ring_binding = args.key_ring.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getKMSCryptoKey:getKMSCryptoKey".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyRing".into(),
                    value: key_ring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKmsCryptoKeyResult {
            crypto_key_backend: o.get_field("cryptoKeyBackend"),
            destroy_scheduled_duration: o.get_field("destroyScheduledDuration"),
            effective_labels: o.get_field("effectiveLabels"),
            id: o.get_field("id"),
            import_only: o.get_field("importOnly"),
            key_access_justifications_policies: o
                .get_field("keyAccessJustificationsPolicies"),
            key_ring: o.get_field("keyRing"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            primaries: o.get_field("primaries"),
            pulumi_labels: o.get_field("pulumiLabels"),
            purpose: o.get_field("purpose"),
            rotation_period: o.get_field("rotationPeriod"),
            skip_initial_version_creation: o.get_field("skipInitialVersionCreation"),
            version_templates: o.get_field("versionTemplates"),
        }
    }
}
