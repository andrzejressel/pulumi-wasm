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
pub mod listener_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerRuleArgs {
        /// The action for the listener rule.
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::ListenerRuleAction,
        >,
        /// The ID or Amazon Resource Name (ARN) of the listener.
        #[builder(into)]
        pub listener_identifier: pulumi_wasm_rust::Output<String>,
        /// The rule match.
        #[builder(into)]
        pub match_: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::ListenerRuleMatch,
        >,
        /// The name of the rule. The name must be unique within the listener. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority assigned to the rule. Each rule for a specific listener must have a unique priority. The lower the priority number the higher the priority.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The ID or Amazon Resource Identifier (ARN) of the service.
        #[builder(into)]
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListenerRuleResult {
        /// The action for the listener rule.
        pub action: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::ListenerRuleAction,
        >,
        /// The ARN for the listener rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Name (ARN) of the listener.
        pub listener_identifier: pulumi_wasm_rust::Output<String>,
        /// The rule match.
        pub match_: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::ListenerRuleMatch,
        >,
        /// The name of the rule. The name must be unique within the listener. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The priority assigned to the rule. Each rule for a specific listener must have a unique priority. The lower the priority number the higher the priority.
        ///
        /// The following arguments are optional:
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// Unique identifier for the listener rule.
        pub rule_id: pulumi_wasm_rust::Output<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service.
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ListenerRuleArgs) -> ListenerRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let listener_identifier_binding = args.listener_identifier.get_inner();
        let match__binding = args.match_.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let service_identifier_binding = args.service_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/listenerRule:ListenerRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "listenerIdentifier".into(),
                    value: &listener_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "match".into(),
                    value: &match__binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "serviceIdentifier".into(),
                    value: &service_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "listenerIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "match".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "ruleId".into(),
                },
                register_interface::ResultField {
                    name: "serviceIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListenerRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            listener_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listenerIdentifier").unwrap(),
            ),
            match_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("match").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleId").unwrap(),
            ),
            service_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceIdentifier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
