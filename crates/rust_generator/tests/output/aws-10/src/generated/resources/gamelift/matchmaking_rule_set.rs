/// Provides a GameLift Matchmaking Rule Set resources.
///
/// ## Import
///
/// GameLift Matchmaking Rule Sets  can be imported using the ID, e.g.,
///
/// ```sh
/// $ pulumi import aws:gamelift/matchmakingRuleSet:MatchmakingRuleSet example <ruleset-id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod matchmaking_rule_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MatchmakingRuleSetArgs {
        /// Name of the matchmaking rule set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// JSON encoded string containing rule set data.
        #[builder(into)]
        pub rule_set_body: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MatchmakingRuleSetResult {
        /// Rule Set ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the matchmaking rule set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// JSON encoded string containing rule set data.
        pub rule_set_body: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MatchmakingRuleSetArgs,
    ) -> MatchmakingRuleSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let rule_set_body_binding_1 = args.rule_set_body.get_output(context);
        let rule_set_body_binding = rule_set_body_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:gamelift/matchmakingRuleSet:MatchmakingRuleSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ruleSetBody".into(),
                    value: &rule_set_body_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MatchmakingRuleSetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            rule_set_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleSetBody"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
