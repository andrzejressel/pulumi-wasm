#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secrets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretsArgs {
        /// Filter string, adhering to the rules in [List-operation filtering](https://cloud.google.com/secret-manager/docs/filtering). List only secrets matching the filter. If filter is empty, all secrets are listed.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecretsResult {
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A list of secrets matching the filter. Structure is defined below.
        pub secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretmanager::GetSecretsSecret>,
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
        let filter_binding = args.filter.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:secretmanager/getSecrets:getSecrets".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSecretsResult {
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            project: o.get_field("project"),
            secrets: o.get_field("secrets"),
        }
    }
}
