#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_state_machine {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStateMachineArgs {
        /// Friendly name of the state machine to match.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetStateMachineResult {
        /// Set to the arn of the state function.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date the state machine was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// Set to the state machine definition.
        pub definition: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The revision identifier for the state machine.
        pub revision_id: pulumi_gestalt_rust::Output<String>,
        /// Set to the role_arn used by the state function.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Set to the current status of the state machine.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetStateMachineArgs,
    ) -> GetStateMachineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sfn/getStateMachine:getStateMachine".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStateMachineResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            creation_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationDate"),
            ),
            definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("definition"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            revision_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revisionId"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
