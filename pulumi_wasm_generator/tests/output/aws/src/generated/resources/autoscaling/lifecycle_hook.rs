/// Provides an AutoScaling Lifecycle Hook resource.
///
/// > **NOTE:** This provider has two types of ways you can add lifecycle hooks - via
/// the `initial_lifecycle_hook` attribute from the
/// `aws.autoscaling.Group`
/// resource, or via this one. Hooks added via this resource will not be added
/// until the autoscaling group has been created, and depending on your
/// capacity
/// settings, after the initial instances have been launched, creating unintended
/// behavior. If you need hooks to run on all instances, add them with
/// `initial_lifecycle_hook` in
/// `aws.autoscaling.Group`,
/// but take care to not duplicate those hooks with this resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foobar:
///     type: aws:autoscaling:Group
///     properties:
///       availabilityZones:
///         - us-west-2a
///       name: test-foobar5
///       healthCheckType: EC2
///       terminationPolicies:
///         - OldestInstance
///       tags:
///         - key: Foo
///           value: foo-bar
///           propagateAtLaunch: true
///   foobarLifecycleHook:
///     type: aws:autoscaling:LifecycleHook
///     name: foobar
///     properties:
///       name: foobar
///       autoscalingGroupName: ${foobar.name}
///       defaultResult: CONTINUE
///       heartbeatTimeout: 2000
///       lifecycleTransition: autoscaling:EC2_INSTANCE_LAUNCHING
///       notificationMetadata:
///         fn::toJSON:
///           foo: bar
///       notificationTargetArn: arn:aws:sqs:us-east-1:444455556666:queue1*
///       roleArn: arn:aws:iam::123456789012:role/S3Access
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AutoScaling Lifecycle Hooks using the role autoscaling_group_name and name separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:autoscaling/lifecycleHook:LifecycleHook test-lifecycle-hook asg-name/lifecycle-hook-name
/// ```
pub mod lifecycle_hook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecycleHookArgs {
        /// Name of the Auto Scaling group to which you want to assign the lifecycle hook
        #[builder(into)]
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The value for this parameter can be either CONTINUE or ABANDON. The default value for this parameter is ABANDON.
        #[builder(into, default)]
        pub default_result: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the amount of time, in seconds, that can elapse before the lifecycle hook times out. When the lifecycle hook times out, Auto Scaling performs the action defined in the DefaultResult parameter
        #[builder(into, default)]
        pub heartbeat_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Instance state to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see [describe-lifecycle-hook-types](https://docs.aws.amazon.com/cli/latest/reference/autoscaling/describe-lifecycle-hook-types.html#examples)
        #[builder(into)]
        pub lifecycle_transition: pulumi_wasm_rust::Output<String>,
        /// Name of the lifecycle hook.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Contains additional information that you want to include any time Auto Scaling sends a message to the notification target.
        #[builder(into, default)]
        pub notification_metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the notification target that Auto Scaling will use to notify you when an instance is in the transition state for the lifecycle hook. This ARN target can be either an SQS queue or an SNS topic.
        #[builder(into, default)]
        pub notification_target_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LifecycleHookResult {
        /// Name of the Auto Scaling group to which you want to assign the lifecycle hook
        pub autoscaling_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The value for this parameter can be either CONTINUE or ABANDON. The default value for this parameter is ABANDON.
        pub default_result: pulumi_wasm_rust::Output<String>,
        /// Defines the amount of time, in seconds, that can elapse before the lifecycle hook times out. When the lifecycle hook times out, Auto Scaling performs the action defined in the DefaultResult parameter
        pub heartbeat_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Instance state to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see [describe-lifecycle-hook-types](https://docs.aws.amazon.com/cli/latest/reference/autoscaling/describe-lifecycle-hook-types.html#examples)
        pub lifecycle_transition: pulumi_wasm_rust::Output<String>,
        /// Name of the lifecycle hook.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Contains additional information that you want to include any time Auto Scaling sends a message to the notification target.
        pub notification_metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the notification target that Auto Scaling will use to notify you when an instance is in the transition state for the lifecycle hook. This ARN target can be either an SQS queue or an SNS topic.
        pub notification_target_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LifecycleHookArgs) -> LifecycleHookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscaling_group_name_binding = args.autoscaling_group_name.get_inner();
        let default_result_binding = args.default_result.get_inner();
        let heartbeat_timeout_binding = args.heartbeat_timeout.get_inner();
        let lifecycle_transition_binding = args.lifecycle_transition.get_inner();
        let name_binding = args.name.get_inner();
        let notification_metadata_binding = args.notification_metadata.get_inner();
        let notification_target_arn_binding = args.notification_target_arn.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/lifecycleHook:LifecycleHook".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscalingGroupName".into(),
                    value: &autoscaling_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "defaultResult".into(),
                    value: &default_result_binding,
                },
                register_interface::ObjectField {
                    name: "heartbeatTimeout".into(),
                    value: &heartbeat_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "lifecycleTransition".into(),
                    value: &lifecycle_transition_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationMetadata".into(),
                    value: &notification_metadata_binding,
                },
                register_interface::ObjectField {
                    name: "notificationTargetArn".into(),
                    value: &notification_target_arn_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoscalingGroupName".into(),
                },
                register_interface::ResultField {
                    name: "defaultResult".into(),
                },
                register_interface::ResultField {
                    name: "heartbeatTimeout".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleTransition".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationMetadata".into(),
                },
                register_interface::ResultField {
                    name: "notificationTargetArn".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LifecycleHookResult {
            autoscaling_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingGroupName").unwrap(),
            ),
            default_result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultResult").unwrap(),
            ),
            heartbeat_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("heartbeatTimeout").unwrap(),
            ),
            lifecycle_transition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleTransition").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationMetadata").unwrap(),
            ),
            notification_target_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationTargetArn").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
        }
    }
}