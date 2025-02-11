#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyStoreArgs {
        /// The ID of the Policy Store.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyStoreResult {
        /// The ARN of the Policy Store.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date the Policy Store was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The date the Policy Store was last updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Validation settings for the policy store.
        pub validation_settings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::verifiedpermissions::GetPolicyStoreValidationSetting,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPolicyStoreArgs,
    ) -> GetPolicyStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:verifiedpermissions/getPolicyStore:getPolicyStore".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPolicyStoreResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            validation_settings: o.get_field("validationSettings"),
        }
    }
}
