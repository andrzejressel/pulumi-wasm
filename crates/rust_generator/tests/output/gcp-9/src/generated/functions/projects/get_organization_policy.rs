#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetOrganizationPolicyArgs,
    ) -> GetOrganizationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let constraint_binding = args.constraint.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:projects/getOrganizationPolicy:getOrganizationPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "constraint".into(),
                    value: constraint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrganizationPolicyResult {
            boolean_policies: o.get_field("booleanPolicies"),
            constraint: o.get_field("constraint"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            list_policies: o.get_field("listPolicies"),
            project: o.get_field("project"),
            restore_policies: o.get_field("restorePolicies"),
            update_time: o.get_field("updateTime"),
            version: o.get_field("version"),
        }
    }
}
