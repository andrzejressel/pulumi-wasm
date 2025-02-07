pub mod get_organization_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationPolicyArgs {
        /// (Required) The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        #[builder(into)]
        pub constraint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project ID.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationPolicyResult {
        pub boolean_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::projects::GetOrganizationPolicyBooleanPolicy>,
        >,
        pub constraint: pulumi_gestalt_rust::Output<String>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub list_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::projects::GetOrganizationPolicyListPolicy>,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub restore_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::projects::GetOrganizationPolicyRestorePolicy>,
        >,
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOrganizationPolicyArgs,
    ) -> GetOrganizationPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let constraint_binding = args.constraint.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:projects/getOrganizationPolicy:getOrganizationPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "constraint".into(),
                    value: &constraint_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrganizationPolicyResult {
            boolean_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("booleanPolicies"),
            ),
            constraint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("constraint"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            list_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listPolicies"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            restore_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restorePolicies"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
