#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_application_assignments {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationAssignmentsArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        #[builder(into, default)]
        pub application_assignments: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApplicationAssignmentsResult {
        /// ARN of the application.
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        pub application_assignments: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetApplicationAssignmentsArgs,
    ) -> GetApplicationAssignmentsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_arn_binding = args.application_arn.get_output(context);
        let application_assignments_binding = args
            .application_assignments
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssoadmin/getApplicationAssignments:getApplicationAssignments"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationArn".into(),
                    value: application_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationAssignments".into(),
                    value: application_assignments_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetApplicationAssignmentsResult {
            application_arn: o.get_field("applicationArn"),
            application_assignments: o.get_field("applicationAssignments"),
            id: o.get_field("id"),
        }
    }
}
