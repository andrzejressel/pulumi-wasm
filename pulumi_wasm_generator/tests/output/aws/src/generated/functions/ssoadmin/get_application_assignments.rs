pub mod get_application_assignments {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationAssignmentsArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        #[builder(into, default)]
        pub application_assignments: pulumi_wasm_rust::Output<
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
        args: GetApplicationAssignmentsArgs,
    ) -> GetApplicationAssignmentsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_arn_binding = args.application_arn.get_inner();
        let application_assignments_binding = args.application_assignments.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getApplicationAssignments:getApplicationAssignments"
                .into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationArn".into(),
                },
                register_interface::ResultField {
                    name: "applicationAssignments".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApplicationAssignmentsResult {
            application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationArn").unwrap(),
            ),
            application_assignments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationAssignments").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}