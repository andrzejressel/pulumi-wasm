/// Provides a CE Anomaly Subscription.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = anomaly_monitor::create(
///         "test",
///         AnomalyMonitorArgs::builder()
///             .monitor_dimension("SERVICE")
///             .monitor_type("DIMENSIONAL")
///             .name("AWSServiceMonitor")
///             .build_struct(),
///     );
///     let testAnomalySubscription = anomaly_subscription::create(
///         "testAnomalySubscription",
///         AnomalySubscriptionArgs::builder()
///             .frequency("DAILY")
///             .monitor_arn_lists(vec!["${test.arn}",])
///             .name("DAILYSUBSCRIPTION")
///             .subscribers(
///                 vec![
///                     AnomalySubscriptionSubscriber::builder().address("abc@example.com").
///                     type ("EMAIL").build_struct(),
///                 ],
///             )
///             .threshold_expression(
///                 AnomalySubscriptionThresholdExpression::builder()
///                     .dimension(
///                         AnomalySubscriptionThresholdExpressionDimension::builder()
///                             .key("ANOMALY_TOTAL_IMPACT_ABSOLUTE")
///                             .matchOptions(vec!["GREATER_THAN_OR_EQUAL",])
///                             .values(vec!["100",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Threshold Expression Example
///
/// ### Using a Percentage Threshold
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = anomaly_subscription::create(
///         "test",
///         AnomalySubscriptionArgs::builder()
///             .frequency("DAILY")
///             .monitor_arn_lists(vec!["${testAwsCeAnomalyMonitor.arn}",])
///             .name("AWSServiceMonitor")
///             .subscribers(
///                 vec![
///                     AnomalySubscriptionSubscriber::builder().address("abc@example.com").
///                     type ("EMAIL").build_struct(),
///                 ],
///             )
///             .threshold_expression(
///                 AnomalySubscriptionThresholdExpression::builder()
///                     .dimension(
///                         AnomalySubscriptionThresholdExpressionDimension::builder()
///                             .key("ANOMALY_TOTAL_IMPACT_PERCENTAGE")
///                             .matchOptions(vec!["GREATER_THAN_OR_EQUAL",])
///                             .values(vec!["100",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using an `and` Expression
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = anomaly_subscription::create(
///         "test",
///         AnomalySubscriptionArgs::builder()
///             .frequency("DAILY")
///             .monitor_arn_lists(vec!["${testAwsCeAnomalyMonitor.arn}",])
///             .name("AWSServiceMonitor")
///             .subscribers(
///                 vec![
///                     AnomalySubscriptionSubscriber::builder().address("abc@example.com").
///                     type ("EMAIL").build_struct(),
///                 ],
///             )
///             .threshold_expression(
///                 AnomalySubscriptionThresholdExpression::builder()
///                     .ands(
///                         vec![
///                             AnomalySubscriptionThresholdExpressionAnd::builder()
///                             .dimension(AnomalySubscriptionThresholdExpressionAndDimension::builder()
///                             .key("ANOMALY_TOTAL_IMPACT_ABSOLUTE")
///                             .matchOptions(vec!["GREATER_THAN_OR_EQUAL",])
///                             .values(vec!["100",]).build_struct()).build_struct(),
///                             AnomalySubscriptionThresholdExpressionAnd::builder()
///                             .dimension(AnomalySubscriptionThresholdExpressionAndDimension::builder()
///                             .key("ANOMALY_TOTAL_IMPACT_PERCENTAGE")
///                             .matchOptions(vec!["GREATER_THAN_OR_EQUAL",])
///                             .values(vec!["50",]).build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### SNS Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let snsTopicPolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .policy_id("__default_policy_ID")
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["SNS:Publish",])
///                     .effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["costalerts.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).resources(vec!["${costAnomalyUpdates.arn}",])
///                     .sid("AWSAnomalyDetectionSNSPublishingPermissions").build_struct(),
///                     GetPolicyDocumentStatement::builder().actions(vec!["SNS:Subscribe",
///                     "SNS:SetTopicAttributes", "SNS:RemovePermission", "SNS:Receive",
///                     "SNS:Publish", "SNS:ListSubscriptionsByTopic",
///                     "SNS:GetTopicAttributes", "SNS:DeleteTopic", "SNS:AddPermission",])
///                     .conditions(vec![GetPolicyDocumentStatementCondition::builder()
///                     .test("StringEquals").values(vec!["${accountId}",])
///                     .variable("AWS:SourceOwner").build_struct(),]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["*",]). type ("AWS").build_struct(),])
///                     .resources(vec!["${costAnomalyUpdates.arn}",])
///                     .sid("__default_statement_ID").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let anomalyMonitor = anomaly_monitor::create(
///         "anomalyMonitor",
///         AnomalyMonitorArgs::builder()
///             .monitor_dimension("SERVICE")
///             .monitor_type("DIMENSIONAL")
///             .name("AWSServiceMonitor")
///             .build_struct(),
///     );
///     let costAnomalyUpdates = topic::create(
///         "costAnomalyUpdates",
///         TopicArgs::builder().name("CostAnomalyUpdates").build_struct(),
///     );
///     let default = topic_policy::create(
///         "default",
///         TopicPolicyArgs::builder()
///             .arn("${costAnomalyUpdates.arn}")
///             .policy("${snsTopicPolicy.json}")
///             .build_struct(),
///     );
///     let realtimeSubscription = anomaly_subscription::create(
///         "realtimeSubscription",
///         AnomalySubscriptionArgs::builder()
///             .frequency("IMMEDIATE")
///             .monitor_arn_lists(vec!["${anomalyMonitor.arn}",])
///             .name("RealtimeAnomalySubscription")
///             .subscribers(
///                 vec![
///                     AnomalySubscriptionSubscriber::builder()
///                     .address("${costAnomalyUpdates.arn}"). type ("SNS").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ce_anomaly_subscription` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:costexplorer/anomalySubscription:AnomalySubscription example AnomalySubscriptionARN
/// ```
pub mod anomaly_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnomalySubscriptionArgs {
        /// The unique identifier for the AWS account in which the anomaly subscription ought to be created.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The frequency that anomaly reports are sent. Valid Values: `DAILY` | `IMMEDIATE` | `WEEKLY`.
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// A list of cost anomaly monitors.
        #[builder(into)]
        pub monitor_arn_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name for the subscription.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A subscriber configuration. Multiple subscribers can be defined.
        #[builder(into)]
        pub subscribers: pulumi_wasm_rust::Output<
            Vec<super::super::types::costexplorer::AnomalySubscriptionSubscriber>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An Expression object used to specify the anomalies that you want to generate alerts for. See Threshold Expression.
        #[builder(into, default)]
        pub threshold_expression: pulumi_wasm_rust::Output<
            Option<
                super::super::types::costexplorer::AnomalySubscriptionThresholdExpression,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AnomalySubscriptionResult {
        /// The unique identifier for the AWS account in which the anomaly subscription ought to be created.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the anomaly subscription.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The frequency that anomaly reports are sent. Valid Values: `DAILY` | `IMMEDIATE` | `WEEKLY`.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// A list of cost anomaly monitors.
        pub monitor_arn_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name for the subscription.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A subscriber configuration. Multiple subscribers can be defined.
        pub subscribers: pulumi_wasm_rust::Output<
            Vec<super::super::types::costexplorer::AnomalySubscriptionSubscriber>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An Expression object used to specify the anomalies that you want to generate alerts for. See Threshold Expression.
        pub threshold_expression: pulumi_wasm_rust::Output<
            super::super::types::costexplorer::AnomalySubscriptionThresholdExpression,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AnomalySubscriptionArgs,
    ) -> AnomalySubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let frequency_binding = args.frequency.get_inner();
        let monitor_arn_lists_binding = args.monitor_arn_lists.get_inner();
        let name_binding = args.name.get_inner();
        let subscribers_binding = args.subscribers.get_inner();
        let tags_binding = args.tags.get_inner();
        let threshold_expression_binding = args.threshold_expression.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:costexplorer/anomalySubscription:AnomalySubscription".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "monitorArnLists".into(),
                    value: &monitor_arn_lists_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subscribers".into(),
                    value: &subscribers_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "thresholdExpression".into(),
                    value: &threshold_expression_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "monitorArnLists".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subscribers".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "thresholdExpression".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AnomalySubscriptionResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            monitor_arn_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorArnLists").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subscribers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscribers").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            threshold_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thresholdExpression").unwrap(),
            ),
        }
    }
}
