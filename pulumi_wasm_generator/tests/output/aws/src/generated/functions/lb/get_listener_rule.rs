pub mod get_listener_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerRuleArgs {
        /// List of actions associated with the rule, sorted by `order`.
        /// Detailed below.
        #[builder(into, default)]
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::lb::GetListenerRuleAction>>,
        >,
        /// ARN of the Listener Rule.
        /// Either `arn` or `listener_arn` must be set.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of conditions associated with the rule.
        /// Detailed below.
        #[builder(into, default)]
        pub conditions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::lb::GetListenerRuleCondition>>,
        >,
        /// ARN of the associated Listener.
        /// Either `arn` or `listener_arn` must be set.
        #[builder(into, default)]
        pub listener_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Priority of the Listener Rule within the Listener.
        /// Must be set if `listener_arn` is set, otherwise must not be set.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<f64>>,
    }
    #[allow(dead_code)]
    pub struct GetListenerRuleResult {
        /// List of actions associated with the rule, sorted by `order`.
        /// Detailed below.
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::lb::GetListenerRuleAction>>,
        >,
        /// ARN of the target group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Set of conditions associated with the rule.
        /// Detailed below.
        pub conditions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::lb::GetListenerRuleCondition>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub listener_arn: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<f64>,
        /// Tags assigned to the Listener Rule.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetListenerRuleArgs) -> GetListenerRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_inner();
        let arn_binding = args.arn.get_inner();
        let conditions_binding = args.conditions.get_inner();
        let listener_arn_binding = args.listener_arn.get_inner();
        let priority_binding = args.priority.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lb/getListenerRule:getListenerRule".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding,
                },
                register_interface::ObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "listenerArn".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetListenerRuleResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            listener_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listenerArn").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
