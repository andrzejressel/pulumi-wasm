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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCryptoKeyVersionsArgs,
    ) -> GetCryptoKeyVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let crypto_key_binding_1 = args.crypto_key.get_output(context);
        let crypto_key_binding = crypto_key_binding_1.get_inner();
        let filter_binding_1 = args.filter.get_output(context);
        let filter_binding = filter_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getCryptoKeyVersions:getCryptoKeyVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKey".into(),
                    value: &crypto_key_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCryptoKeyVersionsResult {
            crypto_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cryptoKey"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            public_keys: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKeys"),
            ),
            versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versions"),
            ),
        }
    }
}
