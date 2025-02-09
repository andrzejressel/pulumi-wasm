#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_rulesets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRulesetsArgs {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::GetRulesetsFilter>,
        >,
        /// Include rule data in response.
        #[builder(into, default)]
        pub include_rules: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRulesetsResult {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::GetRulesetsFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Include rule data in response.
        pub include_rules: pulumi_gestalt_rust::Output<Option<bool>>,
        pub rulesets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetRulesetsRuleset>,
        >,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRulesetsArgs,
    ) -> GetRulesetsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let filter_binding_1 = args.filter.get_output(context);
        let filter_binding = filter_binding_1.get_inner();
        let include_rules_binding_1 = args.include_rules.get_output(context);
        let include_rules_binding = include_rules_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getRulesets:getRulesets".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "includeRules".into(),
                    value: &include_rules_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRulesetsResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            include_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includeRules"),
            ),
            rulesets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rulesets"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
