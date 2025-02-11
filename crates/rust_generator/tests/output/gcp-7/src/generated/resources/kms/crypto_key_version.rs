/// A `CryptoKeyVersion` represents an individual cryptographic key, and the associated key material.
///
///
/// Destroying a cryptoKeyVersion will not delete the resource from the project.
///
///
/// To get more information about CryptoKeyVersion, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions)
/// * How-to Guides
///     * [Creating a key Version](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions/create)
///
/// ## Example Usage
///
/// ### Kms Crypto Key Version Basic
///
///
/// ```yaml
/// resources:
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: keyring-example
///       location: global
///   cryptokey:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: crypto-key-example
///       keyRing: ${keyring.id}
///       rotationPeriod: 7776000s
///   example-key:
///     type: gcp:kms:CryptoKeyVersion
///     properties:
///       cryptoKey: ${cryptokey.id}
/// ```
///
/// ## Import
///
/// CryptoKeyVersion can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, CryptoKeyVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/cryptoKeyVersion:CryptoKeyVersion default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod crypto_key_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CryptoKeyVersionArgs {
        /// The name of the cryptoKey associated with the CryptoKeyVersions.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}'`
        ///
        ///
        /// - - -
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels.
        /// Structure is documented below.
        #[builder(into, default)]
        pub external_protection_level_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::kms::CryptoKeyVersionExternalProtectionLevelOptions,
            >,
        >,
        /// The current state of the CryptoKeyVersion.
        /// Possible values are: `PENDING_GENERATION`, `ENABLED`, `DISABLED`, `DESTROYED`, `DESTROY_SCHEDULED`, `PENDING_IMPORT`, `IMPORT_FAILED`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CryptoKeyVersionResult {
        /// The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
        pub algorithm: pulumi_gestalt_rust::Output<String>,
        /// Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google.
        /// Only provided for key versions with protectionLevel HSM.
        /// Structure is documented below.
        pub attestations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kms::CryptoKeyVersionAttestation>,
        >,
        /// The name of the cryptoKey associated with the CryptoKeyVersions.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}'`
        ///
        ///
        /// - - -
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        /// ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels.
        /// Structure is documented below.
        pub external_protection_level_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::kms::CryptoKeyVersionExternalProtectionLevelOptions,
            >,
        >,
        /// The time this CryptoKeyVersion key material was generated
        pub generate_time: pulumi_gestalt_rust::Output<String>,
        /// The resource name for this CryptoKeyVersion.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion.
        pub protection_level: pulumi_gestalt_rust::Output<String>,
        /// The current state of the CryptoKeyVersion.
        /// Possible values are: `PENDING_GENERATION`, `ENABLED`, `DISABLED`, `DESTROYED`, `DESTROY_SCHEDULED`, `PENDING_IMPORT`, `IMPORT_FAILED`.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CryptoKeyVersionArgs,
    ) -> CryptoKeyVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_binding = args.crypto_key.get_output(context);
        let external_protection_level_options_binding = args
            .external_protection_level_options
            .get_output(context);
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/cryptoKeyVersion:CryptoKeyVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKey".into(),
                    value: &crypto_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalProtectionLevelOptions".into(),
                    value: &external_protection_level_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CryptoKeyVersionResult {
            algorithm: o.get_field("algorithm"),
            attestations: o.get_field("attestations"),
            crypto_key: o.get_field("cryptoKey"),
            external_protection_level_options: o
                .get_field("externalProtectionLevelOptions"),
            generate_time: o.get_field("generateTime"),
            name: o.get_field("name"),
            protection_level: o.get_field("protectionLevel"),
            state: o.get_field("state"),
        }
    }
}
