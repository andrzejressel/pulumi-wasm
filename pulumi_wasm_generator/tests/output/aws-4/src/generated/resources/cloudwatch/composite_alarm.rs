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
pub mod composite_alarm {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CompositeAlarmArgs {
        /// Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. Defaults to `true`.
        #[builder(into, default)]
        pub actions_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Actions will be suppressed if the suppressor alarm is in the ALARM state.
        #[builder(into, default)]
        pub actions_suppressor: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::CompositeAlarmActionsSuppressor>,
        >,
        /// The set of actions to execute when this alarm transitions to the `ALARM` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        #[builder(into, default)]
        pub alarm_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The description for the composite alarm.
        #[builder(into, default)]
        pub alarm_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the composite alarm. This name must be unique within the region.
        #[builder(into)]
        pub alarm_name: pulumi_wasm_rust::Output<String>,
        /// An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For syntax, see [Creating a Composite Alarm](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Create_Composite_Alarm.html). The maximum length is 10240 characters.
        #[builder(into)]
        pub alarm_rule: pulumi_wasm_rust::Output<String>,
        /// The set of actions to execute when this alarm transitions to the `INSUFFICIENT_DATA` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        #[builder(into, default)]
        pub insufficient_data_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The set of actions to execute when this alarm transitions to an `OK` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        #[builder(into, default)]
        pub ok_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to associate with the alarm. Up to 50 tags are allowed. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CompositeAlarmResult {
        /// Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. Defaults to `true`.
        pub actions_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Actions will be suppressed if the suppressor alarm is in the ALARM state.
        pub actions_suppressor: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::CompositeAlarmActionsSuppressor>,
        >,
        /// The set of actions to execute when this alarm transitions to the `ALARM` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        pub alarm_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The description for the composite alarm.
        pub alarm_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the composite alarm. This name must be unique within the region.
        pub alarm_name: pulumi_wasm_rust::Output<String>,
        /// An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For syntax, see [Creating a Composite Alarm](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Create_Composite_Alarm.html). The maximum length is 10240 characters.
        pub alarm_rule: pulumi_wasm_rust::Output<String>,
        /// The ARN of the composite alarm.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The set of actions to execute when this alarm transitions to the `INSUFFICIENT_DATA` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        pub insufficient_data_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The set of actions to execute when this alarm transitions to an `OK` state from any other state. Each action is specified as an ARN. Up to 5 actions are allowed.
        pub ok_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to associate with the alarm. Up to 50 tags are allowed. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: CompositeAlarmArgs) -> CompositeAlarmResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_enabled_binding = args.actions_enabled.get_inner();
        let actions_suppressor_binding = args.actions_suppressor.get_inner();
        let alarm_actions_binding = args.alarm_actions.get_inner();
        let alarm_description_binding = args.alarm_description.get_inner();
        let alarm_name_binding = args.alarm_name.get_inner();
        let alarm_rule_binding = args.alarm_rule.get_inner();
        let insufficient_data_actions_binding = args
            .insufficient_data_actions
            .get_inner();
        let ok_actions_binding = args.ok_actions.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/compositeAlarm:CompositeAlarm".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionsEnabled".into(),
                    value: &actions_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "actionsSuppressor".into(),
                    value: &actions_suppressor_binding,
                },
                register_interface::ObjectField {
                    name: "alarmActions".into(),
                    value: &alarm_actions_binding,
                },
                register_interface::ObjectField {
                    name: "alarmDescription".into(),
                    value: &alarm_description_binding,
                },
                register_interface::ObjectField {
                    name: "alarmName".into(),
                    value: &alarm_name_binding,
                },
                register_interface::ObjectField {
                    name: "alarmRule".into(),
                    value: &alarm_rule_binding,
                },
                register_interface::ObjectField {
                    name: "insufficientDataActions".into(),
                    value: &insufficient_data_actions_binding,
                },
                register_interface::ObjectField {
                    name: "okActions".into(),
                    value: &ok_actions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "actionsSuppressor".into(),
                },
                register_interface::ResultField {
                    name: "alarmActions".into(),
                },
                register_interface::ResultField {
                    name: "alarmDescription".into(),
                },
                register_interface::ResultField {
                    name: "alarmName".into(),
                },
                register_interface::ResultField {
                    name: "alarmRule".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "insufficientDataActions".into(),
                },
                register_interface::ResultField {
                    name: "okActions".into(),
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
        CompositeAlarmResult {
            actions_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionsEnabled").unwrap(),
            ),
            actions_suppressor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionsSuppressor").unwrap(),
            ),
            alarm_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmActions").unwrap(),
            ),
            alarm_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmDescription").unwrap(),
            ),
            alarm_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmName").unwrap(),
            ),
            alarm_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alarmRule").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            insufficient_data_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insufficientDataActions").unwrap(),
            ),
            ok_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("okActions").unwrap(),
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
