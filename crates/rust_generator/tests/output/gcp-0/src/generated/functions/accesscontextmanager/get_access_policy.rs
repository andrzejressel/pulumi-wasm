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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAccessPolicyArgs,
    ) -> GetAccessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let scopes_binding = args.scopes.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:accesscontextmanager/getAccessPolicy:getAccessPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAccessPolicyResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopes"),
            ),
            title: pulumi_gestalt_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
