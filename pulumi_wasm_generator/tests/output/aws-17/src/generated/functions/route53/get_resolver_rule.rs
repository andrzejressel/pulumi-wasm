pub mod get_resolver_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverRuleArgs {
        /// Domain name the desired resolver rule forwards DNS queries for. Conflicts with `resolver_rule_id`.
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the desired resolver rule. Conflicts with `resolver_rule_id`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the outbound resolver endpoint of the desired resolver rule. Conflicts with `resolver_rule_id`.
        #[builder(into, default)]
        pub resolver_endpoint_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the desired resolver rule. Conflicts with `domain_name`, `name`, `resolver_endpoint_id` and `rule_type`.
        #[builder(into, default)]
        pub resolver_rule_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Rule type of the desired resolver rule. Valid values are `FORWARD`, `SYSTEM` and `RECURSIVE`. Conflicts with `resolver_rule_id`.
        #[builder(into, default)]
        pub rule_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resolver rule.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResolverRuleResult {
        /// ARN (Amazon Resource Name) for the resolver rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// When a rule is shared with another AWS account, the account ID of the account that the rule is shared with.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub resolver_endpoint_id: pulumi_wasm_rust::Output<String>,
        pub resolver_rule_id: pulumi_wasm_rust::Output<String>,
        pub rule_type: pulumi_wasm_rust::Output<String>,
        /// Whether the rules is shared and, if so, whether the current account is sharing the rule with another account, or another account is sharing the rule with the current account.
        /// Values are `NOT_SHARED`, `SHARED_BY_ME` or `SHARED_WITH_ME`
        pub share_status: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resolver rule.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResolverRuleArgs,
    ) -> GetResolverRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resolver_endpoint_id_binding = args
            .resolver_endpoint_id
            .get_output(context)
            .get_inner();
        let resolver_rule_id_binding = args
            .resolver_rule_id
            .get_output(context)
            .get_inner();
        let rule_type_binding = args.rule_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverRule:getResolverRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resolverEndpointId".into(),
                    value: &resolver_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "resolverRuleId".into(),
                    value: &resolver_rule_id_binding,
                },
                register_interface::ObjectField {
                    name: "ruleType".into(),
                    value: &rule_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "resolverEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "resolverRuleId".into(),
                },
                register_interface::ResultField {
                    name: "ruleType".into(),
                },
                register_interface::ResultField {
                    name: "shareStatus".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResolverRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            resolver_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resolverEndpointId").unwrap(),
            ),
            resolver_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resolverRuleId").unwrap(),
            ),
            rule_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleType").unwrap(),
            ),
            share_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareStatus").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
