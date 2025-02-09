#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_state_machine_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStateMachineVersionsArgs {
        /// ARN of the State Machine.
        #[builder(into)]
        pub statemachine_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetStateMachineVersionsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub statemachine_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN List identifying the statemachine versions.
        pub statemachine_versions: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetStateMachineVersionsArgs,
    ) -> GetStateMachineVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let statemachine_arn_binding_1 = args.statemachine_arn.get_output(context);
        let statemachine_arn_binding = statemachine_arn_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sfn/getStateMachineVersions:getStateMachineVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "statemachineArn".into(),
                    value: &statemachine_arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStateMachineVersionsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            statemachine_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statemachineArn"),
            ),
            statemachine_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statemachineVersions"),
            ),
        }
    }
}
