pub mod get_principal_application_assignments {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrincipalApplicationAssignmentsArgs {
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        #[builder(into, default)]
        pub application_assignments: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetPrincipalApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
        /// ARN of the instance of IAM Identity Center.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        #[builder(into)]
        pub principal_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrincipalApplicationAssignmentsResult {
        /// List of principals assigned to the application. See the `application_assignments` attribute reference below.
        pub application_assignments: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ssoadmin::GetPrincipalApplicationAssignmentsApplicationAssignment,
                >,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        pub principal_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPrincipalApplicationAssignmentsArgs,
    ) -> GetPrincipalApplicationAssignmentsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_assignments_binding = args
            .application_assignments
            .get_output(context)
            .get_inner();
        let instance_arn_binding = args.instance_arn.get_output(context).get_inner();
        let principal_id_binding = args.principal_id.get_output(context).get_inner();
        let principal_type_binding = args.principal_type.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getPrincipalApplicationAssignments:getPrincipalApplicationAssignments"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationAssignments".into(),
                    value: &application_assignments_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPrincipalApplicationAssignmentsResult {
            application_assignments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationAssignments"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceArn"),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principalType"),
            ),
        }
    }
}
