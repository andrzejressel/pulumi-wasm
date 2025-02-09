#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_crypto_key_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCryptoKeyIamPolicyArgs {
        /// The crypto key ID, in the form
        #[builder(into)]
        pub crypto_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCryptoKeyIamPolicyResult {
        pub crypto_key_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCryptoKeyIamPolicyArgs,
    ) -> GetCryptoKeyIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let crypto_key_id_binding_1 = args.crypto_key_id.get_output(context);
        let crypto_key_id_binding = crypto_key_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:kms/getCryptoKeyIamPolicy:getCryptoKeyIamPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cryptoKeyId".into(),
                    value: &crypto_key_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCryptoKeyIamPolicyResult {
            crypto_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cryptoKeyId"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
        }
    }
}
