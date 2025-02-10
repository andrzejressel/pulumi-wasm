#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPolicyArgs {
        /// The parent of this AccessPolicy in the Cloud Resource Hierarchy. Format: `organizations/{{organization_id}}`
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Folder or project on which this policy is applicable. Format: `folders/{{folder_id}}` or `projects/{{project_number}}`
        #[builder(into, default)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetAccessPolicyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Resource name of the AccessPolicy.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        pub scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Human readable title. Does not affect behavior.
        pub title: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccessPolicyArgs,
    ) -> GetAccessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_binding = args.parent.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:accesscontextmanager/getAccessPolicy:getAccessPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: scopes_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccessPolicyResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            scopes: o.get_field("scopes"),
            title: o.get_field("title"),
        }
    }
}
