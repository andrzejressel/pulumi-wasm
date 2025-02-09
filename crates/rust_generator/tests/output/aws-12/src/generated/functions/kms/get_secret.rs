#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretArgs {
        #[builder(into)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::super::types::kms::GetSecretSecret>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSecretResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kms::GetSecretSecret>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSecretArgs,
    ) -> GetSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let secrets_binding = args.secrets.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kms/getSecret:getSecret".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: secrets_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretResult {
            id: o.get_field("id"),
            secrets: o.get_field("secrets"),
        }
    }
}
