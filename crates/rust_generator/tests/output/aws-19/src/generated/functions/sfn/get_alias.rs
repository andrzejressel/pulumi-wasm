#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAliasArgs {
        /// Description of state machine alias.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the State Machine alias.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the State Machine.
        #[builder(into)]
        pub statemachine_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAliasResult {
        /// ARN identifying the State Machine alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date the state machine Alias was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// Description of state machine alias.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Routing Configuration of state machine alias
        pub routing_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sfn::GetAliasRoutingConfiguration>,
        >,
        pub statemachine_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAliasArgs,
    ) -> GetAliasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let statemachine_arn_binding = args.statemachine_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sfn/getAlias:getAlias".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statemachineArn".into(),
                    value: statemachine_arn_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAliasResult {
            arn: o.get_field("arn"),
            creation_date: o.get_field("creationDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            routing_configurations: o.get_field("routingConfigurations"),
            statemachine_arn: o.get_field("statemachineArn"),
        }
    }
}
