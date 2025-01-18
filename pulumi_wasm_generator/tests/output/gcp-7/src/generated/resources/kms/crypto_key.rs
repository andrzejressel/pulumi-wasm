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
pub mod crypto_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CryptoKeyArgs {
        /// The resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.
        /// The resource name is in the format "projects/*/locations/*/ekmConnections/*" and only applies to "EXTERNAL_VPC" keys.
        #[builder(into, default)]
        pub crypto_key_backend: pulumi_wasm_rust::Output<Option<String>>,
        /// The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.
        /// If not specified at creation time, the default duration is 30 days.
        #[builder(into, default)]
        pub destroy_scheduled_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this key may contain imported versions only.
        #[builder(into, default)]
        pub import_only: pulumi_wasm_rust::Output<Option<bool>>,
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
        pub key_access_justifications_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::kms::CryptoKeyKeyAccessJustificationsPolicy>,
        >,
        /// The KeyRing that this key belongs to.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub key_ring: pulumi_wasm_rust::Output<String>,
        /// Labels with user-defined metadata to apply to this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the CryptoKey.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The immutable purpose of this CryptoKey. See the
        /// [purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)
        /// for possible inputs.
        /// Default value is "ENCRYPT_DECRYPT".
        #[builder(into, default)]
        pub purpose: pulumi_wasm_rust::Output<Option<String>>,
        /// Every time this period passes, generate a new CryptoKeyVersion and set it as the primary.
        /// The first rotation will take place after the specified period. The rotation period has
        /// the format of a decimal number with up to 9 fractional digits, followed by the
        /// letter `s` (seconds). It must be greater than a day (ie, 86400).
        #[builder(into, default)]
        pub rotation_period: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to true, the request will create a CryptoKey without any CryptoKeyVersions.
        /// You must use the `gcp.kms.CryptoKeyVersion` resource to create a new CryptoKeyVersion
        /// or `gcp.kms.KeyRingImportJob` resource to import the CryptoKeyVersion.
        #[builder(into, default)]
        pub skip_initial_version_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// A template describing settings for new crypto key versions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub version_template: pulumi_wasm_rust::Output<
            Option<super::super::types::kms::CryptoKeyVersionTemplate>,
        >,
    }
    #[allow(dead_code)]
    pub struct CryptoKeyResult {
        /// The resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.
        /// The resource name is in the format "projects/*/locations/*/ekmConnections/*" and only applies to "EXTERNAL_VPC" keys.
        pub crypto_key_backend: pulumi_wasm_rust::Output<String>,
        /// The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.
        /// If not specified at creation time, the default duration is 30 days.
        pub destroy_scheduled_duration: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether this key may contain imported versions only.
        pub import_only: pulumi_wasm_rust::Output<bool>,
        /// The policy used for Key Access Justifications Policy Enforcement. If this
        /// field is present and this key is enrolled in Key Access Justifications
        /// Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and
        /// sign operations, and the operation will fail if rejected by the policy. The
        /// policy is defined by specifying zero or more allowed justification codes.
        /// https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes
        /// By default, this field is absent, and all justification codes are allowed.
        /// This field is currently in beta and is subject to change.
        /// Structure is documented below.
        pub key_access_justifications_policy: pulumi_wasm_rust::Output<
            super::super::types::kms::CryptoKeyKeyAccessJustificationsPolicy,
        >,
        /// The KeyRing that this key belongs to.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'`.
        ///
        ///
        /// - - -
        pub key_ring: pulumi_wasm_rust::Output<String>,
        /// Labels with user-defined metadata to apply to this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the CryptoKey.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A copy of the primary CryptoKeyVersion that will be used by cryptoKeys.encrypt when this CryptoKey is given in EncryptRequest.name.
        /// Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be unset.
        /// Structure is documented below.
        pub primaries: pulumi_wasm_rust::Output<
            Vec<super::super::types::kms::CryptoKeyPrimary>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The immutable purpose of this CryptoKey. See the
        /// [purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)
        /// for possible inputs.
        /// Default value is "ENCRYPT_DECRYPT".
        pub purpose: pulumi_wasm_rust::Output<Option<String>>,
        /// Every time this period passes, generate a new CryptoKeyVersion and set it as the primary.
        /// The first rotation will take place after the specified period. The rotation period has
        /// the format of a decimal number with up to 9 fractional digits, followed by the
        /// letter `s` (seconds). It must be greater than a day (ie, 86400).
        pub rotation_period: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to true, the request will create a CryptoKey without any CryptoKeyVersions.
        /// You must use the `gcp.kms.CryptoKeyVersion` resource to create a new CryptoKeyVersion
        /// or `gcp.kms.KeyRingImportJob` resource to import the CryptoKeyVersion.
        pub skip_initial_version_creation: pulumi_wasm_rust::Output<Option<bool>>,
        /// A template describing settings for new crypto key versions.
        /// Structure is documented below.
        pub version_template: pulumi_wasm_rust::Output<
            super::super::types::kms::CryptoKeyVersionTemplate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CryptoKeyArgs) -> CryptoKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let crypto_key_backend_binding = args.crypto_key_backend.get_inner();
        let destroy_scheduled_duration_binding = args
            .destroy_scheduled_duration
            .get_inner();
        let import_only_binding = args.import_only.get_inner();
        let key_access_justifications_policy_binding = args
            .key_access_justifications_policy
            .get_inner();
        let key_ring_binding = args.key_ring.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let purpose_binding = args.purpose.get_inner();
        let rotation_period_binding = args.rotation_period.get_inner();
        let skip_initial_version_creation_binding = args
            .skip_initial_version_creation
            .get_inner();
        let version_template_binding = args.version_template.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:kms/cryptoKey:CryptoKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKeyBackend".into(),
                    value: &crypto_key_backend_binding,
                },
                register_interface::ObjectField {
                    name: "destroyScheduledDuration".into(),
                    value: &destroy_scheduled_duration_binding,
                },
                register_interface::ObjectField {
                    name: "importOnly".into(),
                    value: &import_only_binding,
                },
                register_interface::ObjectField {
                    name: "keyAccessJustificationsPolicy".into(),
                    value: &key_access_justifications_policy_binding,
                },
                register_interface::ObjectField {
                    name: "keyRing".into(),
                    value: &key_ring_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding,
                },
                register_interface::ObjectField {
                    name: "rotationPeriod".into(),
                    value: &rotation_period_binding,
                },
                register_interface::ObjectField {
                    name: "skipInitialVersionCreation".into(),
                    value: &skip_initial_version_creation_binding,
                },
                register_interface::ObjectField {
                    name: "versionTemplate".into(),
                    value: &version_template_binding,
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
                    name: "importOnly".into(),
                },
                register_interface::ResultField {
                    name: "keyAccessJustificationsPolicy".into(),
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
                    name: "versionTemplate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CryptoKeyResult {
            crypto_key_backend: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cryptoKeyBackend").unwrap(),
            ),
            destroy_scheduled_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destroyScheduledDuration").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            import_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importOnly").unwrap(),
            ),
            key_access_justifications_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyAccessJustificationsPolicy").unwrap(),
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
            version_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionTemplate").unwrap(),
            ),
        }
    }
}
