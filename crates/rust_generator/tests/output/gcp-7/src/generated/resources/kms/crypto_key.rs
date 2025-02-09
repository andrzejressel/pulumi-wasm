/// A `CryptoKey` represents a logical key that can be used for cryptographic operations.
///
///
/// > **Note:** CryptoKeys cannot be deleted from Google Cloud Platform.
/// Destroying a provider-managed CryptoKey will remove it from state
/// and delete all CryptoKeyVersions, rendering the key unusable, but *will
/// not delete the resource from the project.* When the provider destroys these keys,
/// any data previously encrypted with these keys will be irrecoverable.
/// For this reason, it is strongly recommended that you use Pulumi's [protect resource option](https://www.pulumi.com/docs/concepts/options/protect/).
///
///
/// To get more information about CryptoKey, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys)
/// * How-to Guides
///     * [Creating a key](https://cloud.google.com/kms/docs/creating-keys#create_a_key)
///
/// ## Example Usage
///
/// ### Kms Crypto Key Basic
///
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   example-key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       rotationPeriod: 7776000s
/// ```
/// ### Kms Crypto Key Asymmetric Sign
///
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   example-asymmetric-sign-key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       purpose: ASYMMETRIC_SIGN
///       versionTemplate:
///         algorithm: EC_SIGN_P384_SHA384
/// ```
///
/// ## Import
///
/// CryptoKey can be imported using any of these accepted formats:
///
/// * `{{key_ring}}/cryptoKeys/{{name}}`
///
/// * `{{key_ring}}/{{name}}`
///
/// When using the `pulumi import` command, CryptoKey can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/cryptoKey:CryptoKey default {{key_ring}}/cryptoKeys/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/cryptoKey:CryptoKey default {{key_ring}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod crypto_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CryptoKeyArgs {
        /// The resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.
        /// The resource name is in the format "projects/*/locations/*/ekmConnections/*" and only applies to "EXTERNAL_VPC" keys.
        #[builder(into, default)]
        pub crypto_key_backend: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.
        /// If not specified at creation time, the default duration is 30 days.
        #[builder(into, default)]
        pub destroy_scheduled_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether this key may contain imported versions only.
        #[builder(into, default)]
        pub import_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The policy used for Key Access Justifications Policy Enforcement. If this
        /// field is present and this key is enrolled in Key Access Justifications
        /// Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and
        /// sign operations, and the operation will fail if rejected by the policy. The
        /// policy is defined by specifying zero or more allowed justification codes.
        /// https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes
        /// By default, this field is absent, and all justification codes are allowed.
        /// This field is currently in beta and is subject to change.
        /// Structure is documented below.
        #[builder(into, default)]
        pub key_access_justifications_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kms::CryptoKeyKeyAccessJustificationsPolicy>,
        >,
        /// The KeyRing that this key belongs to.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub key_ring: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Labels with user-defined metadata to apply to this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the CryptoKey.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The immutable purpose of this CryptoKey. See the
        /// [purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)
        /// for possible inputs.
        /// Default value is "ENCRYPT_DECRYPT".
        #[builder(into, default)]
        pub purpose: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Every time this period passes, generate a new CryptoKeyVersion and set it as the primary.
        /// The first rotation will take place after the specified period. The rotation period has
        /// the format of a decimal number with up to 9 fractional digits, followed by the
        /// letter `s` (seconds). It must be greater than a day (ie, 86400).
        #[builder(into, default)]
        pub rotation_period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set to true, the request will create a CryptoKey without any CryptoKeyVersions.
        /// You must use the `gcp.kms.CryptoKeyVersion` resource to create a new CryptoKeyVersion
        /// or `gcp.kms.KeyRingImportJob` resource to import the CryptoKeyVersion.
        #[builder(into, default)]
        pub skip_initial_version_creation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A template describing settings for new crypto key versions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub version_template: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kms::CryptoKeyVersionTemplate>,
        >,
    }
    #[allow(dead_code)]
    pub struct CryptoKeyResult {
        /// The resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.
        /// The resource name is in the format "projects/*/locations/*/ekmConnections/*" and only applies to "EXTERNAL_VPC" keys.
        pub crypto_key_backend: pulumi_gestalt_rust::Output<String>,
        /// The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.
        /// If not specified at creation time, the default duration is 30 days.
        pub destroy_scheduled_duration: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether this key may contain imported versions only.
        pub import_only: pulumi_gestalt_rust::Output<bool>,
        /// The policy used for Key Access Justifications Policy Enforcement. If this
        /// field is present and this key is enrolled in Key Access Justifications
        /// Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and
        /// sign operations, and the operation will fail if rejected by the policy. The
        /// policy is defined by specifying zero or more allowed justification codes.
        /// https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes
        /// By default, this field is absent, and all justification codes are allowed.
        /// This field is currently in beta and is subject to change.
        /// Structure is documented below.
        pub key_access_justifications_policy: pulumi_gestalt_rust::Output<
            super::super::types::kms::CryptoKeyKeyAccessJustificationsPolicy,
        >,
        /// The KeyRing that this key belongs to.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'`.
        ///
        ///
        /// - - -
        pub key_ring: pulumi_gestalt_rust::Output<String>,
        /// Labels with user-defined metadata to apply to this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the CryptoKey.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A copy of the primary CryptoKeyVersion that will be used by cryptoKeys.encrypt when this CryptoKey is given in EncryptRequest.name.
        /// Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be unset.
        /// Structure is documented below.
        pub primaries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kms::CryptoKeyPrimary>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The immutable purpose of this CryptoKey. See the
        /// [purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)
        /// for possible inputs.
        /// Default value is "ENCRYPT_DECRYPT".
        pub purpose: pulumi_gestalt_rust::Output<Option<String>>,
        /// Every time this period passes, generate a new CryptoKeyVersion and set it as the primary.
        /// The first rotation will take place after the specified period. The rotation period has
        /// the format of a decimal number with up to 9 fractional digits, followed by the
        /// letter `s` (seconds). It must be greater than a day (ie, 86400).
        pub rotation_period: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set to true, the request will create a CryptoKey without any CryptoKeyVersions.
        /// You must use the `gcp.kms.CryptoKeyVersion` resource to create a new CryptoKeyVersion
        /// or `gcp.kms.KeyRingImportJob` resource to import the CryptoKeyVersion.
        pub skip_initial_version_creation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A template describing settings for new crypto key versions.
        /// Structure is documented below.
        pub version_template: pulumi_gestalt_rust::Output<
            super::super::types::kms::CryptoKeyVersionTemplate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CryptoKeyArgs,
    ) -> CryptoKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_backend_binding = args.crypto_key_backend.get_output(context);
        let destroy_scheduled_duration_binding = args
            .destroy_scheduled_duration
            .get_output(context);
        let import_only_binding = args.import_only.get_output(context);
        let key_access_justifications_policy_binding = args
            .key_access_justifications_policy
            .get_output(context);
        let key_ring_binding = args.key_ring.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let purpose_binding = args.purpose.get_output(context);
        let rotation_period_binding = args.rotation_period.get_output(context);
        let skip_initial_version_creation_binding = args
            .skip_initial_version_creation
            .get_output(context);
        let version_template_binding = args.version_template.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/cryptoKey:CryptoKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKeyBackend".into(),
                    value: crypto_key_backend_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destroyScheduledDuration".into(),
                    value: destroy_scheduled_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importOnly".into(),
                    value: import_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyAccessJustificationsPolicy".into(),
                    value: key_access_justifications_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyRing".into(),
                    value: key_ring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purpose".into(),
                    value: purpose_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rotationPeriod".into(),
                    value: rotation_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipInitialVersionCreation".into(),
                    value: skip_initial_version_creation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionTemplate".into(),
                    value: version_template_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CryptoKeyResult {
            crypto_key_backend: o.get_field("cryptoKeyBackend"),
            destroy_scheduled_duration: o.get_field("destroyScheduledDuration"),
            effective_labels: o.get_field("effectiveLabels"),
            import_only: o.get_field("importOnly"),
            key_access_justifications_policy: o
                .get_field("keyAccessJustificationsPolicy"),
            key_ring: o.get_field("keyRing"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            primaries: o.get_field("primaries"),
            pulumi_labels: o.get_field("pulumiLabels"),
            purpose: o.get_field("purpose"),
            rotation_period: o.get_field("rotationPeriod"),
            skip_initial_version_creation: o.get_field("skipInitialVersionCreation"),
            version_template: o.get_field("versionTemplate"),
        }
    }
}
