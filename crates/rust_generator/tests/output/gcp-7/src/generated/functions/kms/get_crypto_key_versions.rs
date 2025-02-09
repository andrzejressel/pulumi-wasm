#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_crypto_key_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeyVersionsArgs {
        /// The `id` of the Google Cloud Platform CryptoKey to which the key version belongs. This is also the `id` field of the
        /// `gcp.kms.CryptoKey` resource/datasource.
        #[builder(into)]
        pub crypto_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The filter argument is used to add a filter query parameter that limits which versions are retrieved by the data source: ?filter={{filter}}. When no value is provided there is no filtering.
        ///
        /// Example filter values if filtering on name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}/cryptoKeyVersions.
        ///
        /// * `"name:my-key-"` will retrieve cryptoKeyVersions that contain "my-key-" anywhere in their name.
        /// * `"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1/cryptoKeyVersions/my-version-1"` will only retrieve a key with that exact name.
        ///
        /// [See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeyVersionsResult {
        pub crypto_key: pulumi_gestalt_rust::Output<String>,
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub public_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeyVersionsPublicKey>,
        >,
        /// A list of all the retrieved crypto key versions from the provided crypto key. This list is influenced by the provided filter argument.
        pub versions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeyVersionsVersion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCryptoKeyVersionsArgs,
    ) -> GetCryptoKeyVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_binding = args.crypto_key.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:kms/getCryptoKeyVersions:getCryptoKeyVersions".into(),
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
        GetCryptoKeyVersionsResult {
            crypto_key: o.get_field("cryptoKey"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            public_keys: o.get_field("publicKeys"),
            versions: o.get_field("versions"),
        }
    }
}
