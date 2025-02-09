/// A `KeyRingImportJob` can be used to create `CryptoKeys` and `CryptoKeyVersions` using pre-existing
/// key material, generated outside of Cloud KMS. A `KeyRingImportJob` expires 3 days after it is created.
/// Once expired, Cloud KMS will no longer be able to import or unwrap any key material that
/// was wrapped with the `KeyRingImportJob`'s public key.
///
///
/// > **Note:** KeyRingImportJobs cannot be deleted from Google Cloud Platform.
/// Destroying a provider-managed KeyRingImportJob will remove it from state but
/// *will not delete the resource from the project.*
///
///
/// To get more information about KeyRingImportJob, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.importJobs)
/// * How-to Guides
///     * [Importing a key](https://cloud.google.com/kms/docs/importing-a-key)
///
/// ## Example Usage
///
/// ## Import
///
/// KeyRingImportJob can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, KeyRingImportJob can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/keyRingImportJob:KeyRingImportJob default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_ring_import_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyRingImportJobArgs {
        /// It must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub import_job_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The wrapping method to be used for incoming key material.
        /// Possible values are: `RSA_OAEP_3072_SHA1_AES_256`, `RSA_OAEP_4096_SHA1_AES_256`.
        #[builder(into)]
        pub import_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The KeyRing that this import job belongs to.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'`.
        #[builder(into)]
        pub key_ring: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The protection level of the ImportJob. This must match the protectionLevel of the
        /// versionTemplate on the CryptoKey you attempt to import into.
        /// Possible values are: `SOFTWARE`, `HSM`, `EXTERNAL`.
        #[builder(into)]
        pub protection_level: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeyRingImportJobResult {
        /// Statement that was generated and signed by the key creator (for example, an HSM) at key creation time.
        /// Use this statement to verify attributes of the key as stored on the HSM, independently of Google.
        /// Only present if the chosen ImportMethod is one with a protection level of HSM.
        /// Structure is documented below.
        pub attestations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kms::KeyRingImportJobAttestation>,
        >,
        /// The time at which this resource is scheduled for expiration and can no longer be used.
        /// This is in RFC3339 text format.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// It must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}
        ///
        ///
        /// - - -
        pub import_job_id: pulumi_gestalt_rust::Output<String>,
        /// The wrapping method to be used for incoming key material.
        /// Possible values are: `RSA_OAEP_3072_SHA1_AES_256`, `RSA_OAEP_4096_SHA1_AES_256`.
        pub import_method: pulumi_gestalt_rust::Output<String>,
        /// The KeyRing that this import job belongs to.
        /// Format: `'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'`.
        pub key_ring: pulumi_gestalt_rust::Output<String>,
        /// The resource name for this ImportJob in the format projects/*/locations/*/keyRings/*/importJobs/*.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The protection level of the ImportJob. This must match the protectionLevel of the
        /// versionTemplate on the CryptoKey you attempt to import into.
        /// Possible values are: `SOFTWARE`, `HSM`, `EXTERNAL`.
        pub protection_level: pulumi_gestalt_rust::Output<String>,
        /// The public key with which to wrap key material prior to import. Only returned if state is `ACTIVE`.
        /// Structure is documented below.
        pub public_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kms::KeyRingImportJobPublicKey>,
        >,
        /// The current state of the ImportJob, indicating if it can be used.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyRingImportJobArgs,
    ) -> KeyRingImportJobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let import_job_id_binding = args.import_job_id.get_output(context);
        let import_method_binding = args.import_method.get_output(context);
        let key_ring_binding = args.key_ring.get_output(context);
        let protection_level_binding = args.protection_level.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/keyRingImportJob:KeyRingImportJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importJobId".into(),
                    value: import_job_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importMethod".into(),
                    value: import_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyRing".into(),
                    value: key_ring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectionLevel".into(),
                    value: protection_level_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyRingImportJobResult {
            attestations: o.get_field("attestations"),
            expire_time: o.get_field("expireTime"),
            import_job_id: o.get_field("importJobId"),
            import_method: o.get_field("importMethod"),
            key_ring: o.get_field("keyRing"),
            name: o.get_field("name"),
            protection_level: o.get_field("protectionLevel"),
            public_keys: o.get_field("publicKeys"),
            state: o.get_field("state"),
        }
    }
}
