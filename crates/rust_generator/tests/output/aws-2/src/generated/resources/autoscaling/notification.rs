/// Provides an AutoScaling Group with Notification support, via SNS Topics. Each of
/// the `notifications` map to a [Notification Configuration](https://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_DescribeNotificationConfigurations.html) inside Amazon Web
/// Services, and are applied to each AutoScaling Group you supply.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bar = group::create(
///         "bar",
///         GroupArgs::builder().name("foobar1-test").build_struct(),
///     );
///     let example = topic::create(
///         "example",
///         TopicArgs::builder().name("example-topic").build_struct(),
///     );
///     let exampleNotifications = notification::create(
///         "exampleNotifications",
///         NotificationArgs::builder()
///             .group_names(vec!["${bar.name}", "${foo.name}",])
///             .notifications(
///                 vec![
///                     "autoscaling:EC2_INSTANCE_LAUNCH",
///                     "autoscaling:EC2_INSTANCE_TERMINATE",
///                     "autoscaling:EC2_INSTANCE_LAUNCH_ERROR",
///                     "autoscaling:EC2_INSTANCE_TERMINATE_ERROR",
///                 ],
///             )
///             .topic_arn("${example.arn}")
///             .build_struct(),
///     );
///     let foo = group::create(
///         "foo",
///         GroupArgs::builder().name("barfoo-test").build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationArgs {
        /// List of AutoScaling Group Names
        #[builder(into)]
        pub group_names: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// List of Notification Types that trigger
        /// notifications. Acceptable values are documented [in the AWS documentation here](https://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_NotificationConfiguration.html)
        #[builder(into)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Topic ARN for notifications to be sent through
        #[builder(into)]
        pub topic_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NotificationResult {
        /// List of AutoScaling Group Names
        pub group_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of Notification Types that trigger
        /// notifications. Acceptable values are documented [in the AWS documentation here](https://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_NotificationConfiguration.html)
        pub notifications: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Topic ARN for notifications to be sent through
        pub topic_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationArgs,
    ) -> NotificationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_names_binding = args.group_names.get_output(context);
        let notifications_binding = args.notifications.get_output(context);
        let topic_arn_binding = args.topic_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:autoscaling/notification:Notification".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupNames".into(),
                    value: group_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notifications".into(),
                    value: notifications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicArn".into(),
                    value: topic_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationResult {
            group_names: o.get_field("groupNames"),
            notifications: o.get_field("notifications"),
            topic_arn: o.get_field("topicArn"),
        }
    }
}
