/// Manages a ServiceBus Subscription Rule.
///
/// ## Example Usage
///
/// ### SQL Filter)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-servicebus-subscription-rule-sql
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: tfex-servicebus-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       tags:
///         source: example
///   exampleTopic:
///     type: azure:servicebus:Topic
///     name: example
///     properties:
///       name: tfex_servicebus_topic
///       namespaceId: ${exampleNamespace.id}
///       enablePartitioning: true
///   exampleSubscription:
///     type: azure:servicebus:Subscription
///     name: example
///     properties:
///       name: tfex_servicebus_subscription
///       topicId: ${exampleTopic.id}
///       maxDeliveryCount: 1
///   exampleSubscriptionRule:
///     type: azure:servicebus:SubscriptionRule
///     name: example
///     properties:
///       name: tfex_servicebus_rule
///       subscriptionId: ${exampleSubscription.id}
///       filterType: SqlFilter
///       sqlFilter: colour = 'red'
/// ```
///
///
/// ### Correlation Filter)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-servicebus-subscription-rule-cor
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: tfex-servicebus-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       tags:
///         source: example
///   exampleTopic:
///     type: azure:servicebus:Topic
///     name: example
///     properties:
///       name: tfex_servicebus_topic
///       namespaceId: ${exampleNamespace.id}
///       enablePartitioning: true
///   exampleSubscription:
///     type: azure:servicebus:Subscription
///     name: example
///     properties:
///       name: tfex_servicebus_subscription
///       topicId: ${exampleTopic.id}
///       maxDeliveryCount: 1
///   exampleSubscriptionRule:
///     type: azure:servicebus:SubscriptionRule
///     name: example
///     properties:
///       name: tfex_servicebus_rule
///       subscriptionId: ${exampleSubscription.id}
///       filterType: CorrelationFilter
///       correlationFilter:
///         correlationId: high
///         label: red
///         properties:
///           customProperty: value
/// ```
///
/// ## Import
///
/// Service Bus Subscription Rule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/subscriptionRule:SubscriptionRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/topics/sntopic1/subscriptions/sbsub1/rules/sbrule1
/// ```
///
pub mod subscription_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionRuleArgs {
        /// Represents set of actions written in SQL language-based syntax that is performed against a BrokeredMessage.
        #[builder(into, default)]
        pub action: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `correlation_filter` block as documented below to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `CorrelationFilter`.
        #[builder(into, default)]
        pub correlation_filter: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::eventhub::SubscriptionRuleCorrelationFilter>,
        >,
        /// Type of filter to be applied to a BrokeredMessage. Possible values are `SqlFilter` and `CorrelationFilter`.
        #[builder(into)]
        pub filter_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the ServiceBus Subscription Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Represents a filter written in SQL language-based syntax that to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `SqlFilter`.
        #[builder(into, default)]
        pub sql_filter: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the ServiceBus Subscription in which this Rule should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subscription_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionRuleResult {
        /// Represents set of actions written in SQL language-based syntax that is performed against a BrokeredMessage.
        pub action: pulumi_wasm_rust::Output<Option<String>>,
        /// A `correlation_filter` block as documented below to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `CorrelationFilter`.
        pub correlation_filter: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::SubscriptionRuleCorrelationFilter>,
        >,
        /// Type of filter to be applied to a BrokeredMessage. Possible values are `SqlFilter` and `CorrelationFilter`.
        pub filter_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the ServiceBus Subscription Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Represents a filter written in SQL language-based syntax that to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `SqlFilter`.
        pub sql_filter: pulumi_wasm_rust::Output<Option<String>>,
        pub sql_filter_compatibility_level: pulumi_wasm_rust::Output<i32>,
        /// The ID of the ServiceBus Subscription in which this Rule should be created. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubscriptionRuleArgs,
    ) -> SubscriptionRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let correlation_filter_binding = args
            .correlation_filter
            .get_output(context)
            .get_inner();
        let filter_type_binding = args.filter_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let sql_filter_binding = args.sql_filter.get_output(context).get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/subscriptionRule:SubscriptionRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "correlationFilter".into(),
                    value: &correlation_filter_binding,
                },
                register_interface::ObjectField {
                    name: "filterType".into(),
                    value: &filter_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sqlFilter".into(),
                    value: &sql_filter_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriptionRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(o.extract_field("action")),
            correlation_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("correlationFilter"),
            ),
            filter_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filterType"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            sql_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sqlFilter"),
            ),
            sql_filter_compatibility_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sqlFilterCompatibilityLevel"),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
        }
    }
}
