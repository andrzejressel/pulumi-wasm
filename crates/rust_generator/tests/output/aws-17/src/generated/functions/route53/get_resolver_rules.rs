#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resolver_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverRulesArgs {
        /// Regex string to filter resolver rule names.
        /// The filtering is done locally, so could have a performance impact if the result is large.
        /// This argument should be used along with other arguments to limit the number of results returned.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When the desired resolver rules are shared with another AWS account, the account ID of the account that the rules are shared with.
        #[builder(into, default)]
        pub owner_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the outbound resolver endpoint for the desired resolver rules.
        #[builder(into, default)]
        pub resolver_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rule type of the desired resolver rules. Valid values are `FORWARD`, `SYSTEM` and `RECURSIVE`.
        #[builder(into, default)]
        pub rule_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the desired resolver rules are shared and, if so, whether the current account is sharing the rules with another account, or another account is sharing the rules with the current account. Valid values are `NOT_SHARED`, `SHARED_BY_ME` or `SHARED_WITH_ME`
        #[builder(into, default)]
        pub share_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResolverRulesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        pub owner_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub resolver_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// IDs of the matched resolver rules.
        pub resolver_rule_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub rule_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub share_status: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResolverRulesArgs,
    ) -> GetResolverRulesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_regex_binding = args.name_regex.get_output(context).get_inner();
        let owner_id_binding = args.owner_id.get_output(context).get_inner();
        let resolver_endpoint_id_binding = args
            .resolver_endpoint_id
            .get_output(context)
            .get_inner();
        let rule_type_binding = args.rule_type.get_output(context).get_inner();
        let share_status_binding = args.share_status.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverRules:getResolverRules".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "ownerId".into(),
                    value: &owner_id_binding,
                },
                register_interface::ObjectField {
                    name: "resolverEndpointId".into(),
                    value: &resolver_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "ruleType".into(),
                    value: &rule_type_binding,
                },
                register_interface::ObjectField {
                    name: "shareStatus".into(),
                    value: &share_status_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverRulesResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name_regex: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameRegex"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            resolver_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resolverEndpointId"),
            ),
            resolver_rule_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resolverRuleIds"),
            ),
            rule_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ruleType"),
            ),
            share_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareStatus"),
            ),
        }
    }
}
