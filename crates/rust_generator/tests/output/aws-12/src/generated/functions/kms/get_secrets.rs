#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secrets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretsArgs {
        /// One or more encrypted payload definitions from the KMS service. See the Secret Definitions below.
        #[builder(into)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::kms::GetSecretsSecret>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSecretsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Map containing each `secret` `name` as the key with its decrypted plaintext value
        pub plaintext: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetSecretsSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretsArgs,
    ) -> GetSecretsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let secrets_binding_1 = args.secrets.get_output(context);
        let secrets_binding = secrets_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getSecrets:getSecrets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSecretsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            plaintext: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("plaintext"),
            ),
            secrets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secrets"),
            ),
        }
    }
}
