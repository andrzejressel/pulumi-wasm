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
        context: &pulumi_gestalt_rust::Context,
        args: GetStateMachineArgs,
    ) -> GetStateMachineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sfn/getStateMachine:getStateMachine".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetStateMachineResult {
            arn: o.get_field("arn"),
            creation_date: o.get_field("creationDate"),
            definition: o.get_field("definition"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            revision_id: o.get_field("revisionId"),
            role_arn: o.get_field("roleArn"),
            status: o.get_field("status"),
        }
    }
}
