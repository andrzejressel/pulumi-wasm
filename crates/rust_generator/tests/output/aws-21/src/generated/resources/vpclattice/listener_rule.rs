/// Resource for managing an AWS VPC Lattice Listener Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:vpclattice:ListenerRule
///     properties:
///       name: example
///       listenerIdentifier: ${exampleAwsVpclatticeListener.listenerId}
///       serviceIdentifier: ${exampleAwsVpclatticeService.id}
///       priority: 20
///       match:
///         httpMatch:
///           headerMatches:
///             - name: example-header
///               caseSensitive: false
///               match:
///                 exact: example-contains
///           pathMatch:
///             caseSensitive: true
///             match:
///               prefix: /example-path
///       action:
///         forward:
///           targetGroups:
///             - targetGroupIdentifier: ${example.id}
///               weight: 1
///             - targetGroupIdentifier: ${example2.id}
///               weight: 2
/// ```
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:vpclattice:ListenerRule
///     properties:
///       name: example
///       listenerIdentifier: ${example.listenerId}
///       serviceIdentifier: ${exampleAwsVpclatticeService.id}
///       priority: 10
///       match:
///         httpMatch:
///           pathMatch:
///             caseSensitive: false
///             match:
///               exact: /example-path
///       action:
///         fixedResponse:
///           statusCode: 404
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Listener Rule using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/listenerRule:ListenerRule example service123/listener456/rule789
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod listener_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerRuleArgs {
        /// The action for the listener rule.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::vpclattice::ListenerRuleAction,
        >,
        /// The ID or Amazon Resource Name (ARN) of the listener.
        #[builder(into)]
        pub listener_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The rule match.
        #[builder(into)]
        pub match_: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::vpclattice::ListenerRuleMatch,
        >,
        /// The name of the rule. The name must be unique within the listener. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority assigned to the rule. Each rule for a specific listener must have a unique priority. The lower the priority number the higher the priority.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID or Amazon Resource Identifier (ARN) of the service.
        #[builder(into)]
        pub service_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListenerRuleResult {
        /// The action for the listener rule.
        pub action: pulumi_gestalt_rust::Output<
            super::super::types::vpclattice::ListenerRuleAction,
        >,
        /// The ARN for the listener rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID or Amazon Resource Name (ARN) of the listener.
        pub listener_identifier: pulumi_gestalt_rust::Output<String>,
        /// The rule match.
        pub match_: pulumi_gestalt_rust::Output<
            super::super::types::vpclattice::ListenerRuleMatch,
        >,
        /// The name of the rule. The name must be unique within the listener. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority assigned to the rule. Each rule for a specific listener must have a unique priority. The lower the priority number the higher the priority.
        ///
        /// The following arguments are optional:
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// Unique identifier for the listener rule.
        pub rule_id: pulumi_gestalt_rust::Output<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service.
        pub service_identifier: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListenerRuleArgs,
    ) -> ListenerRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let listener_identifier_binding = args.listener_identifier.get_output(context);
        let match__binding = args.match_.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let service_identifier_binding = args.service_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/listenerRule:ListenerRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerIdentifier".into(),
                    value: listener_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "match".into(),
                    value: match__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceIdentifier".into(),
                    value: service_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ListenerRuleResult {
            action: o.get_field("action"),
            arn: o.get_field("arn"),
            listener_identifier: o.get_field("listenerIdentifier"),
            match_: o.get_field("match"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            rule_id: o.get_field("ruleId"),
            service_identifier: o.get_field("serviceIdentifier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
