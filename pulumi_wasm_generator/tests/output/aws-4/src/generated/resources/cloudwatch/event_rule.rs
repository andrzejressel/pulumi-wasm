/// Provides an EventBridge Rule resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   console:
///     type: aws:cloudwatch:EventRule
///     properties:
///       name: capture-aws-sign-in
///       description: Capture each AWS Console Sign In
///       eventPattern:
///         fn::toJSON:
///           detail-type:
///             - AWS Console Sign In via CloudTrail
///   sns:
///     type: aws:cloudwatch:EventTarget
///     properties:
///       rule: ${console.name}
///       targetId: SendToSNS
///       arn: ${awsLogins.arn}
///   awsLogins:
///     type: aws:sns:Topic
///     name: aws_logins
///     properties:
///       name: aws-console-logins
///   default:
///     type: aws:sns:TopicPolicy
///     properties:
///       arn: ${awsLogins.arn}
///       policy: ${snsTopicPolicy.json}
/// variables:
///   snsTopicPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - SNS:Publish
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///             resources:
///               - ${awsLogins.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge Rules using the `event_bus_name/rule_name` (if you omit `event_bus_name`, the `default` event bus will be used). For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventRule:EventRule console example-event-bus/capture-console-sign-in
/// ```
pub mod event_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventRuleArgs {
        /// The description of the rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name or ARN of the event bus to associate with this rule.
        /// If you omit this, the `default` event bus is used.
        #[builder(into, default)]
        pub event_bus_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The event pattern described a JSON object. At least one of `schedule_expression` or `event_pattern` is required. See full documentation of [Events and Event Patterns in EventBridge](https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html) for details. **Note**: The event pattern size is 2048 by default but it is adjustable up to 4096 characters by submitting a service quota increase request. See [Amazon EventBridge quotas](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-quota.html) for details.
        #[builder(into, default)]
        pub event_pattern: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Used to delete managed rules created by AWS. Defaults to `false`.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether the rule should be enabled.
        /// Defaults to `true`.
        /// Conflicts with `state`.
        #[builder(into, default)]
        pub is_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the rule. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. **Note**: Due to the length of the generated suffix, must be 38 characters or less.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) associated with the role that is used for target invocation.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The scheduling expression. For example, `cron(0 20 * * ? *)` or `rate(5 minutes)`. At least one of `schedule_expression` or `event_pattern` is required. Can only be used on the default event bus. For more information, refer to the AWS documentation [Schedule Expressions for Rules](https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html).
        #[builder(into, default)]
        pub schedule_expression: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// State of the rule.
        /// Valid values are `DISABLED`, `ENABLED`, and `ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS`.
        /// When state is `ENABLED`, the rule is enabled for all events except those delivered by CloudTrail.
        /// To also enable the rule for events delivered by CloudTrail, set `state` to `ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS`.
        /// Defaults to `ENABLED`.
        /// Conflicts with `is_enabled`.
        ///
        /// **NOTE:** The rule state  `ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS` cannot be used in conjunction with the `schedule_expression` argument.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventRuleResult {
        /// The Amazon Resource Name (ARN) of the rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The description of the rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name or ARN of the event bus to associate with this rule.
        /// If you omit this, the `default` event bus is used.
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The event pattern described a JSON object. At least one of `schedule_expression` or `event_pattern` is required. See full documentation of [Events and Event Patterns in EventBridge](https://docs.aws.amazon.com/eventbridge/latest/userguide/eventbridge-and-event-patterns.html) for details. **Note**: The event pattern size is 2048 by default but it is adjustable up to 4096 characters by submitting a service quota increase request. See [Amazon EventBridge quotas](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-quota.html) for details.
        pub event_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to delete managed rules created by AWS. Defaults to `false`.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the rule should be enabled.
        /// Defaults to `true`.
        /// Conflicts with `state`.
        pub is_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the rule. If omitted, this provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. **Note**: Due to the length of the generated suffix, must be 38 characters or less.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) associated with the role that is used for target invocation.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The scheduling expression. For example, `cron(0 20 * * ? *)` or `rate(5 minutes)`. At least one of `schedule_expression` or `event_pattern` is required. Can only be used on the default event bus. For more information, refer to the AWS documentation [Schedule Expressions for Rules](https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html).
        pub schedule_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the rule.
        /// Valid values are `DISABLED`, `ENABLED`, and `ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS`.
        /// When state is `ENABLED`, the rule is enabled for all events except those delivered by CloudTrail.
        /// To also enable the rule for events delivered by CloudTrail, set `state` to `ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS`.
        /// Defaults to `ENABLED`.
        /// Conflicts with `is_enabled`.
        ///
        /// **NOTE:** The rule state  `ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS` cannot be used in conjunction with the `schedule_expression` argument.
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EventRuleArgs,
    ) -> EventRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let event_bus_name_binding = args.event_bus_name.get_output(context).get_inner();
        let event_pattern_binding = args.event_pattern.get_output(context).get_inner();
        let force_destroy_binding = args.force_destroy.get_output(context).get_inner();
        let is_enabled_binding = args.is_enabled.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let schedule_expression_binding = args
            .schedule_expression
            .get_output(context)
            .get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventRule:EventRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventBusName".into(),
                    value: &event_bus_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventPattern".into(),
                    value: &event_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "isEnabled".into(),
                    value: &is_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleExpression".into(),
                    value: &schedule_expression_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            event_bus_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventBusName"),
            ),
            event_pattern: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventPattern"),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            is_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isEnabled"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            schedule_expression: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduleExpression"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
