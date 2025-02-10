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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lifecycle_hook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecycleHookArgs {
        /// Name of the Auto Scaling group to which you want to assign the lifecycle hook
        #[builder(into)]
        pub autoscaling_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The value for this parameter can be either CONTINUE or ABANDON. The default value for this parameter is ABANDON.
        #[builder(into, default)]
        pub default_result: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the amount of time, in seconds, that can elapse before the lifecycle hook times out. When the lifecycle hook times out, Auto Scaling performs the action defined in the DefaultResult parameter
        #[builder(into, default)]
        pub heartbeat_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Instance state to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see [describe-lifecycle-hook-types](https://docs.aws.amazon.com/cli/latest/reference/autoscaling/describe-lifecycle-hook-types.html#examples)
        #[builder(into)]
        pub lifecycle_transition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the lifecycle hook.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Contains additional information that you want to include any time Auto Scaling sends a message to the notification target.
        #[builder(into, default)]
        pub notification_metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the notification target that Auto Scaling will use to notify you when an instance is in the transition state for the lifecycle hook. This ARN target can be either an SQS queue or an SNS topic.
        #[builder(into, default)]
        pub notification_target_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LifecycleHookResult {
        /// Name of the Auto Scaling group to which you want to assign the lifecycle hook
        pub autoscaling_group_name: pulumi_gestalt_rust::Output<String>,
        /// Defines the action the Auto Scaling group should take when the lifecycle hook timeout elapses or if an unexpected failure occurs. The value for this parameter can be either CONTINUE or ABANDON. The default value for this parameter is ABANDON.
        pub default_result: pulumi_gestalt_rust::Output<String>,
        /// Defines the amount of time, in seconds, that can elapse before the lifecycle hook times out. When the lifecycle hook times out, Auto Scaling performs the action defined in the DefaultResult parameter
        pub heartbeat_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Instance state to which you want to attach the lifecycle hook. For a list of lifecycle hook types, see [describe-lifecycle-hook-types](https://docs.aws.amazon.com/cli/latest/reference/autoscaling/describe-lifecycle-hook-types.html#examples)
        pub lifecycle_transition: pulumi_gestalt_rust::Output<String>,
        /// Name of the lifecycle hook.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Contains additional information that you want to include any time Auto Scaling sends a message to the notification target.
        pub notification_metadata: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the notification target that Auto Scaling will use to notify you when an instance is in the transition state for the lifecycle hook. This ARN target can be either an SQS queue or an SNS topic.
        pub notification_target_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LifecycleHookArgs,
    ) -> LifecycleHookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscaling_group_name_binding = args
            .autoscaling_group_name
            .get_output(context);
        let default_result_binding = args.default_result.get_output(context);
        let heartbeat_timeout_binding = args.heartbeat_timeout.get_output(context);
        let lifecycle_transition_binding = args.lifecycle_transition.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_metadata_binding = args
            .notification_metadata
            .get_output(context);
        let notification_target_arn_binding = args
            .notification_target_arn
            .get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:autoscaling/lifecycleHook:LifecycleHook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscalingGroupName".into(),
                    value: autoscaling_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultResult".into(),
                    value: default_result_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "heartbeatTimeout".into(),
                    value: heartbeat_timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lifecycleTransition".into(),
                    value: lifecycle_transition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationMetadata".into(),
                    value: notification_metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationTargetArn".into(),
                    value: notification_target_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LifecycleHookResult {
            autoscaling_group_name: o.get_field("autoscalingGroupName"),
            default_result: o.get_field("defaultResult"),
            heartbeat_timeout: o.get_field("heartbeatTimeout"),
            lifecycle_transition: o.get_field("lifecycleTransition"),
            name: o.get_field("name"),
            notification_metadata: o.get_field("notificationMetadata"),
            notification_target_arn: o.get_field("notificationTargetArn"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
