/// Provides a CloudWatch Composite Alarm resource.
///
/// > **NOTE:** An alarm (composite or metric) cannot be destroyed when there are other composite alarms depending on it. This can lead to a cyclical dependency on update, as the provider will unsuccessfully attempt to destroy alarms before updating the rule. Consider using `depends_on`, references to alarm names, and two-stage updates.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:CompositeAlarm
///     properties:
///       alarmDescription: This is a composite alarm!
///       alarmName: example-composite-alarm
///       alarmActions: ${exampleAwsSnsTopic.arn}
///       okActions: ${exampleAwsSnsTopic.arn}
///       alarmRule: |
///         ALARM(${alpha.alarmName}) OR
///         ALARM(${bravo.alarmName})
///       actionsSuppressor:
///         alarm: suppressor-alarm
///         extensionPeriod: 10
///         waitPeriod: 20
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a CloudWatch Composite Alarm using the `alarm_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/compositeAlarm:CompositeAlarm test my-alarm
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod composite_alarm {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CompositeAlarmArgs {
        /// Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. Defaults to `true`.
        #[builder(into, default)]
        pub actions_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Actions will be suppressed if the suppressor alarm is in the ALARM state.
        #[builder(into, default)]
        pub actions_suppressor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::CompositeAlarmActionsSuppressor>,
        >,
        /// The set of actions to execute when this alarm transitions to the `ALARM` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        #[builder(into, default)]
        pub alarm_actions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The description for the composite alarm.
        #[builder(into, default)]
        pub alarm_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name for the composite alarm. This name must be unique within the region.
        #[builder(into)]
        pub alarm_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For syntax, see [Creating a Composite Alarm](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Create_Composite_Alarm.html). The maximum length is 10240 characters.
        #[builder(into)]
        pub alarm_rule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The set of actions to execute when this alarm transitions to the `INSUFFICIENT_DATA` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        #[builder(into, default)]
        pub insufficient_data_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The set of actions to execute when this alarm transitions to an `OK` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        #[builder(into, default)]
        pub ok_actions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to associate with the alarm. Up to 50 tags are allowed. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CompositeAlarmResult {
        /// Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. Defaults to `true`.
        pub actions_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Actions will be suppressed if the suppressor alarm is in the ALARM state.
        pub actions_suppressor: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::CompositeAlarmActionsSuppressor>,
        >,
        /// The set of actions to execute when this alarm transitions to the `ALARM` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        pub alarm_actions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The description for the composite alarm.
        pub alarm_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name for the composite alarm. This name must be unique within the region.
        pub alarm_name: pulumi_gestalt_rust::Output<String>,
        /// An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For syntax, see [Creating a Composite Alarm](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Create_Composite_Alarm.html). The maximum length is 10240 characters.
        pub alarm_rule: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the composite alarm.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The set of actions to execute when this alarm transitions to the `INSUFFICIENT_DATA` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        pub insufficient_data_actions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The set of actions to execute when this alarm transitions to an `OK` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        pub ok_actions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A map of tags to associate with the alarm. Up to 50 tags are allowed. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CompositeAlarmArgs,
    ) -> CompositeAlarmResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_enabled_binding = args.actions_enabled.get_output(context);
        let actions_suppressor_binding = args.actions_suppressor.get_output(context);
        let alarm_actions_binding = args.alarm_actions.get_output(context);
        let alarm_description_binding = args.alarm_description.get_output(context);
        let alarm_name_binding = args.alarm_name.get_output(context);
        let alarm_rule_binding = args.alarm_rule.get_output(context);
        let insufficient_data_actions_binding = args
            .insufficient_data_actions
            .get_output(context);
        let ok_actions_binding = args.ok_actions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/compositeAlarm:CompositeAlarm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionsEnabled".into(),
                    value: &actions_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actionsSuppressor".into(),
                    value: &actions_suppressor_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alarmActions".into(),
                    value: &alarm_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alarmDescription".into(),
                    value: &alarm_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alarmName".into(),
                    value: &alarm_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alarmRule".into(),
                    value: &alarm_rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "insufficientDataActions".into(),
                    value: &insufficient_data_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "okActions".into(),
                    value: &ok_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CompositeAlarmResult {
            actions_enabled: o.get_field("actionsEnabled"),
            actions_suppressor: o.get_field("actionsSuppressor"),
            alarm_actions: o.get_field("alarmActions"),
            alarm_description: o.get_field("alarmDescription"),
            alarm_name: o.get_field("alarmName"),
            alarm_rule: o.get_field("alarmRule"),
            arn: o.get_field("arn"),
            insufficient_data_actions: o.get_field("insufficientDataActions"),
            ok_actions: o.get_field("okActions"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
