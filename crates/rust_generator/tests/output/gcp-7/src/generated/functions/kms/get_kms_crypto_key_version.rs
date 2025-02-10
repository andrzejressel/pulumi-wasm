#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kms_crypto_key_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyVersionArgs {
        /// The `id` of the Google Cloud Platform CryptoKey to which the key version belongs. This is also the `id` field of the
        /// `gcp.kms.CryptoKey` resource/datasource.
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version number for this CryptoKeyVersion. Defaults to `1`.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetKmsCryptoKeyVersionResult {
        /// The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
        pub algorithm: pulumi_gestalt_rust::Output<String>,
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion. See the [protection_level reference](https://cloud.google.com/kms/docs/reference/rest/v1/ProtectionLevel) for possible outputs.
        pub protection_level: pulumi_gestalt_rust::Output<String>,
        /// If the enclosing CryptoKey has purpose `ASYMMETRIC_SIGN` or `ASYMMETRIC_DECRYPT`, this block contains details about the public key associated to this CryptoKeyVersion. Structure is documented below.
        pub public_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetKmsCryptoKeyVersionPublicKey>,
        >,
        /// The current state of the CryptoKeyVersion. See the [state reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions#CryptoKeyVersion.CryptoKeyVersionState) for possible outputs.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKmsCryptoKeyVersionArgs,
    ) -> GetKmsCryptoKeyVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_binding = args.crypto_key.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getKMSCryptoKeyVersion:getKMSCryptoKeyVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKey".into(),
                    value: crypto_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKmsCryptoKeyVersionResult {
            algorithm: o.get_field("algorithm"),
            crypto_key: o.get_field("cryptoKey"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            protection_level: o.get_field("protectionLevel"),
            public_keys: o.get_field("publicKeys"),
            state: o.get_field("state"),
            version: o.get_field("version"),
        }
    }
}
