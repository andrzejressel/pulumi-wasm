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
        context: &pulumi_gestalt_rust::Context,
        args: GetSecretsArgs,
    ) -> GetSecretsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let secrets_binding = args.secrets.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kms/getSecrets:getSecrets".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretsResult {
            id: o.get_field("id"),
            plaintext: o.get_field("plaintext"),
            secrets: o.get_field("secrets"),
        }
    }
}
