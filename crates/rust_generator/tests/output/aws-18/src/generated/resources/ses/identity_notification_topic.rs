/// Resource for managing SES Identity Notification Topics
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = identity_notification_topic::create(
///         "test",
///         IdentityNotificationTopicArgs::builder()
///             .identity("${example.domain}")
///             .include_original_headers(true)
///             .notification_type("Bounce")
///             .topic_arn("${exampleAwsSnsTopic.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Identity Notification Topics using the ID of the record. The ID is made up as `IDENTITY|TYPE` where `IDENTITY` is the SES Identity and `TYPE` is the Notification Type. For example:
///
/// ```sh
/// $ pulumi import aws:ses/identityNotificationTopic:IdentityNotificationTopic test 'example.com|Bounce'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_notification_topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityNotificationTopicArgs {
        /// The identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN).
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether SES should include original email headers in SNS notifications of this type. `false` by default.
        #[builder(into, default)]
        pub include_original_headers: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The type of notifications that will be published to the specified Amazon SNS topic. Valid Values: `Bounce`, `Complaint` or `Delivery`.
        #[builder(into)]
        pub notification_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Amazon SNS topic. Can be set to `""` (an empty string) to disable publishing.
        #[builder(into, default)]
        pub topic_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IdentityNotificationTopicResult {
        /// The identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN).
        pub identity: pulumi_gestalt_rust::Output<String>,
        /// Whether SES should include original email headers in SNS notifications of this type. `false` by default.
        pub include_original_headers: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The type of notifications that will be published to the specified Amazon SNS topic. Valid Values: `Bounce`, `Complaint` or `Delivery`.
        pub notification_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon SNS topic. Can be set to `""` (an empty string) to disable publishing.
        pub topic_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityNotificationTopicArgs,
    ) -> IdentityNotificationTopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let include_original_headers_binding = args
            .include_original_headers
            .get_output(context);
        let notification_type_binding = args.notification_type.get_output(context);
        let topic_arn_binding = args.topic_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/identityNotificationTopic:IdentityNotificationTopic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeOriginalHeaders".into(),
                    value: include_original_headers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationType".into(),
                    value: notification_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicArn".into(),
                    value: topic_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityNotificationTopicResult {
            identity: o.get_field("identity"),
            include_original_headers: o.get_field("includeOriginalHeaders"),
            notification_type: o.get_field("notificationType"),
            topic_arn: o.get_field("topicArn"),
        }
    }
}
