#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_listener_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerRuleArgs {
        /// List of actions associated with the rule, sorted by `order`.
        /// Detailed below.
        #[builder(into, default)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::lb::GetListenerRuleAction>>,
        >,
        /// ARN of the Listener Rule.
        /// Either `arn` or `listener_arn` must be set.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of conditions associated with the rule.
        /// Detailed below.
        #[builder(into, default)]
        pub conditions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::lb::GetListenerRuleCondition>>,
        >,
        /// ARN of the associated Listener.
        /// Either `arn` or `listener_arn` must be set.
        #[builder(into, default)]
        pub listener_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Priority of the Listener Rule within the Listener.
        /// Must be set if `listener_arn` is set, otherwise must not be set.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
    }
    #[allow(dead_code)]
    pub struct GetListenerRuleResult {
        /// List of actions associated with the rule, sorted by `order`.
        /// Detailed below.
        pub actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::lb::GetListenerRuleAction>>,
        >,
        /// ARN of the target group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Set of conditions associated with the rule.
        /// Detailed below.
        pub conditions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::lb::GetListenerRuleCondition>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub listener_arn: pulumi_gestalt_rust::Output<String>,
        pub priority: pulumi_gestalt_rust::Output<f64>,
        /// Tags assigned to the Listener Rule.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetListenerRuleArgs,
    ) -> GetListenerRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let arn_binding = args.arn.get_output(context);
        let conditions_binding = args.conditions.get_output(context);
        let listener_arn_binding = args.listener_arn.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lb/getListenerRule:getListenerRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetListenerRuleResult {
            actions: o.get_field("actions"),
            arn: o.get_field("arn"),
            conditions: o.get_field("conditions"),
            id: o.get_field("id"),
            listener_arn: o.get_field("listenerArn"),
            priority: o.get_field("priority"),
            tags: o.get_field("tags"),
        }
    }
}
