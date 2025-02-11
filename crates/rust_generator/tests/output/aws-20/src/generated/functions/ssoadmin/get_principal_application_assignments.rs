#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_principal_application_assignments {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrincipalApplicationAssignmentsArgs {
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        #[builder(into, default)]
        pub application_assignments: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetPrincipalApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
        /// ARN of the instance of IAM Identity Center.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        #[builder(into)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrincipalApplicationAssignmentsResult {
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        pub application_assignments: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetPrincipalApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPrincipalApplicationAssignmentsArgs,
    ) -> GetPrincipalApplicationAssignmentsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_assignments_binding = args
            .application_assignments
            .get_output(context);
        let instance_arn_binding = args.instance_arn.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let principal_type_binding = args.principal_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssoadmin/getPrincipalApplicationAssignments:getPrincipalApplicationAssignments"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationAssignments".into(),
                    value: &application_assignments_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPrincipalApplicationAssignmentsResult {
            application_assignments: o.get_field("applicationAssignments"),
            id: o.get_field("id"),
            instance_arn: o.get_field("instanceArn"),
            principal_id: o.get_field("principalId"),
            principal_type: o.get_field("principalType"),
        }
    }
}
