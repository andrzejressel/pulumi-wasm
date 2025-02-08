#[allow(clippy::doc_lazy_continuation)]
pub mod get_crypto_keys {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeysArgs {
        /// The filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}. When no value is provided there is no filtering.
        ///
        /// Example filter values if filtering on name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}/cryptoKeys/{{cryptoKey}}.
        ///
        /// * `"name:my-key-"` will retrieve keys that contain "my-key-" anywhere in their name.
        /// * `"name=projects/my-project/locations/global/keyRings/my-key-ring/cryptoKeys/my-key-1"` will only retrieve a key with that exact name.
        ///
        /// [See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The key ring that the keys belongs to. Format: 'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'.,
        #[builder(into)]
        pub key_ring: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeysResult {
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key_ring: pulumi_gestalt_rust::Output<String>,
        /// A list of all the retrieved keys from the provided key ring. This list is influenced by the provided filter argument.
        pub keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetCryptoKeysKey>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCryptoKeysArgs,
    ) -> GetCryptoKeysResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let key_ring_binding = args.key_ring.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getCryptoKeys:getCryptoKeys".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "keyRing".into(),
                    value: &key_ring_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCryptoKeysResult {
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_ring: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyRing"),
            ),
            keys: pulumi_gestalt_rust::__private::into_domain(o.extract_field("keys")),
        }
    }
}
