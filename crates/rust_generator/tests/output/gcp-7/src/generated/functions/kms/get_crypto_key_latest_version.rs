#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_crypto_key_latest_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeyLatestVersionArgs {
        /// The `id` of the Google Cloud Platform CryptoKey to which the key version belongs. This is also the `id` field of the
        /// `gcp.kms.CryptoKey` resource/datasource.
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The filter argument is used to add a filter query parameter that limits which type of cryptoKeyVersion is retrieved as the latest by the data source: ?filter={{filter}}. When no value is provided there is no filtering.
        ///
        /// Example filter values if filtering on state.
        ///
        /// * `"state:ENABLED"` will retrieve the latest cryptoKeyVersion that has the state "ENABLED".
        ///
        /// [See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeyLatestVersionResult {
        /// The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports.
        pub algorithm: pulumi_gestalt_rust::Output<String>,
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion. See the [protection_level reference](https://cloud.google.com/kms/docs/reference/rest/v1/ProtectionLevel) for possible outputs.
        pub protection_level: pulumi_gestalt_rust::Output<String>,
        /// If the enclosing CryptoKey has purpose `ASYMMETRIC_SIGN` or `ASYMMETRIC_DECRYPT`, this block contains details about the public key associated to this CryptoKeyVersion. Structure is documented below.
        pub public_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeyLatestVersionPublicKey>,
        >,
        /// The current state of the latest CryptoKeyVersion. See the [state reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions#CryptoKeyVersion.CryptoKeyVersionState) for possible outputs.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCryptoKeyLatestVersionArgs,
    ) -> GetCryptoKeyLatestVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_binding = args.crypto_key.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getCryptoKeyLatestVersion:getCryptoKeyLatestVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKey".into(),
                    value: crypto_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCryptoKeyLatestVersionResult {
            algorithm: o.get_field("algorithm"),
            crypto_key: o.get_field("cryptoKey"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            protection_level: o.get_field("protectionLevel"),
            public_keys: o.get_field("publicKeys"),
            state: o.get_field("state"),
            version: o.get_field("version"),
        }
    }
}
