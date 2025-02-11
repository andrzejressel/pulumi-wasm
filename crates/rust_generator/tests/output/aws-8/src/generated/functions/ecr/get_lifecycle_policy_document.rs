#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_lifecycle_policy_document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLifecyclePolicyDocumentArgs {
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ecr::GetLifecyclePolicyDocumentRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLifecyclePolicyDocumentResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The above arguments serialized as a standard JSON policy document.
        pub json: pulumi_gestalt_rust::Output<String>,
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ecr::GetLifecyclePolicyDocumentRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLifecyclePolicyDocumentArgs,
    ) -> GetLifecyclePolicyDocumentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecr/getLifecyclePolicyDocument:getLifecyclePolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLifecyclePolicyDocumentResult {
            id: o.get_field("id"),
            json: o.get_field("json"),
            rules: o.get_field("rules"),
        }
    }
}
