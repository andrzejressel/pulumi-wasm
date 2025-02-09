/// Provides a CE Anomaly Subscription.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ```yaml
/// resources:
///   costAnomalyUpdates:
///     type: aws:sns:Topic
///     name: cost_anomaly_updates
///     properties:
///       name: CostAnomalyUpdates
///   default:
///     type: aws:sns:TopicPolicy
///     properties:
///       arn: ${costAnomalyUpdates.arn}
///       policy: ${snsTopicPolicy.json}
///   anomalyMonitor:
///     type: aws:costexplorer:AnomalyMonitor
///     name: anomaly_monitor
///     properties:
///       name: AWSServiceMonitor
///       monitorType: DIMENSIONAL
///       monitorDimension: SERVICE
///   realtimeSubscription:
///     type: aws:costexplorer:AnomalySubscription
///     name: realtime_subscription
///     properties:
///       name: RealtimeAnomalySubscription
///       frequency: IMMEDIATE
///       monitorArnLists:
///         - ${anomalyMonitor.arn}
///       subscribers:
///         - type: SNS
///           address: ${costAnomalyUpdates.arn}
///     options:
///       dependsOn:
///         - ${default}
/// variables:
///   snsTopicPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         policyId: __default_policy_ID
///         statements:
///           - sid: AWSAnomalyDetectionSNSPublishingPermissions
///             actions:
///               - SNS:Publish
///             effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - costalerts.amazonaws.com
///             resources:
///               - ${costAnomalyUpdates.arn}
///           - sid: __default_statement_ID
///             actions:
///               - SNS:Subscribe
///               - SNS:SetTopicAttributes
///               - SNS:RemovePermission
///               - SNS:Receive
///               - SNS:Publish
///               - SNS:ListSubscriptionsByTopic
///               - SNS:GetTopicAttributes
///               - SNS:DeleteTopic
///               - SNS:AddPermission
///             conditions:
///               - test: StringEquals
///                 variable: AWS:SourceOwner
///                 values:
///                   - ${accountId}
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             resources:
///               - ${costAnomalyUpdates.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ce_anomaly_subscription` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:costexplorer/anomalySubscription:AnomalySubscription example AnomalySubscriptionARN
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod anomaly_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnomalySubscriptionArgs {
        /// The unique identifier for the AWS account in which the anomaly subscription ought to be created.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The frequency that anomaly reports are sent. Valid Values: `DAILY` | `IMMEDIATE` | `WEEKLY`.
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of cost anomaly monitors.
        #[builder(into)]
        pub monitor_arn_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name for the subscription.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A subscriber configuration. Multiple subscribers can be defined.
        #[builder(into)]
        pub subscribers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::costexplorer::AnomalySubscriptionSubscriber>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An Expression object used to specify the anomalies that you want to generate alerts for. See Threshold Expression.
        #[builder(into, default)]
        pub threshold_expression: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::costexplorer::AnomalySubscriptionThresholdExpression,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AnomalySubscriptionResult {
        /// The unique identifier for the AWS account in which the anomaly subscription ought to be created.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the anomaly subscription.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The frequency that anomaly reports are sent. Valid Values: `DAILY` | `IMMEDIATE` | `WEEKLY`.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// A list of cost anomaly monitors.
        pub monitor_arn_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name for the subscription.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A subscriber configuration. Multiple subscribers can be defined.
        pub subscribers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::costexplorer::AnomalySubscriptionSubscriber>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An Expression object used to specify the anomalies that you want to generate alerts for. See Threshold Expression.
        pub threshold_expression: pulumi_gestalt_rust::Output<
            super::super::types::costexplorer::AnomalySubscriptionThresholdExpression,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AnomalySubscriptionArgs,
    ) -> AnomalySubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let monitor_arn_lists_binding = args.monitor_arn_lists.get_output(context);
        let name_binding = args.name.get_output(context);
        let subscribers_binding = args.subscribers.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let threshold_expression_binding = args.threshold_expression.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:costexplorer/anomalySubscription:AnomalySubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frequency".into(),
                    value: frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitorArnLists".into(),
                    value: monitor_arn_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscribers".into(),
                    value: subscribers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thresholdExpression".into(),
                    value: threshold_expression_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AnomalySubscriptionResult {
            account_id: o.get_field("accountId"),
            arn: o.get_field("arn"),
            frequency: o.get_field("frequency"),
            monitor_arn_lists: o.get_field("monitorArnLists"),
            name: o.get_field("name"),
            subscribers: o.get_field("subscribers"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            threshold_expression: o.get_field("thresholdExpression"),
        }
    }
}
