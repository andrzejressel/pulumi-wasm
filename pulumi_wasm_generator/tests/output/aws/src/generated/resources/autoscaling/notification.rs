/// Provides an AutoScaling Group with Notification support, via SNS Topics. Each of
/// the `notifications` map to a [Notification Configuration](https://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_DescribeNotificationConfigurations.html) inside Amazon Web
/// Services, and are applied to each AutoScaling Group you supply.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod notification {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationArgs {
        /// List of AutoScaling Group Names
        #[builder(into)]
        pub group_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of Notification Types that trigger
        /// notifications. Acceptable values are documented [in the AWS documentation here](https://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_NotificationConfiguration.html)
        #[builder(into)]
        pub notifications: pulumi_wasm_rust::Output<Vec<String>>,
        /// Topic ARN for notifications to be sent through
        #[builder(into)]
        pub topic_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NotificationResult {
        /// List of AutoScaling Group Names
        pub group_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of Notification Types that trigger
        /// notifications. Acceptable values are documented [in the AWS documentation here](https://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_NotificationConfiguration.html)
        pub notifications: pulumi_wasm_rust::Output<Vec<String>>,
        /// Topic ARN for notifications to be sent through
        pub topic_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NotificationArgs) -> NotificationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_names_binding = args.group_names.get_inner();
        let notifications_binding = args.notifications.get_inner();
        let topic_arn_binding = args.topic_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscaling/notification:Notification".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupNames".into(),
                    value: &group_names_binding,
                },
                register_interface::ObjectField {
                    name: "notifications".into(),
                    value: &notifications_binding,
                },
                register_interface::ObjectField {
                    name: "topicArn".into(),
                    value: &topic_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupNames".into(),
                },
                register_interface::ResultField {
                    name: "notifications".into(),
                },
                register_interface::ResultField {
                    name: "topicArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NotificationResult {
            group_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupNames").unwrap(),
            ),
            notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notifications").unwrap(),
            ),
            topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topicArn").unwrap(),
            ),
        }
    }
}
