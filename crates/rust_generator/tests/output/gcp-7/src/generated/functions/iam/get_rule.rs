#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRuleArgs {
        /// The name of the Role to lookup in the form `roles/{ROLE_NAME}`, `organizations/{ORGANIZATION_ID}/roles/{ROLE_NAME}` or `projects/{PROJECT_ID}/roles/{ROLE_NAME}`
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// specifies the list of one or more permissions to include in the custom role, such as - `iam.roles.get`
        pub included_permissions: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// indicates the stage of a role in the launch lifecycle, such as `GA`, `BETA` or `ALPHA`.
        pub stage: pulumi_gestalt_rust::Output<String>,
        /// is a friendly title for the role, such as "Role Viewer"
        pub title: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRuleArgs,
    ) -> GetRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:iam/getRule:getRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRuleResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            included_permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includedPermissions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            stage: pulumi_gestalt_rust::__private::into_domain(o.extract_field("stage")),
            title: pulumi_gestalt_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
