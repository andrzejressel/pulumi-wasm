pub mod get_application_assignments {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationAssignmentsArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        #[builder(into, default)]
        pub application_assignments: pulumi_wasm_rust::InputOrOutput<
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
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        pub application_assignments: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetApplicationAssignmentsArgs,
    ) -> GetApplicationAssignmentsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_arn_binding = args
            .application_arn
            .get_output(context)
            .get_inner();
        let application_assignments_binding = args
            .application_assignments
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getApplicationAssignments:getApplicationAssignments"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding,
                },
                register_interface::ObjectField {
                    name: "applicationAssignments".into(),
                    value: &application_assignments_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetApplicationAssignmentsResult {
            application_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationArn"),
            ),
            application_assignments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationAssignments"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
