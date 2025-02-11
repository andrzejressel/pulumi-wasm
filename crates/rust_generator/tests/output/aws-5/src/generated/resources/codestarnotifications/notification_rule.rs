/// Provides a CodeStar Notifications Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   code:
///     type: aws:codecommit:Repository
///     properties:
///       repositoryName: example-code-repo
///   notif:
///     type: aws:sns:Topic
///     properties:
///       name: notification
///   default:
///     type: aws:sns:TopicPolicy
///     properties:
///       arn: ${notif.arn}
///       policy: ${notifAccess.json}
///   commits:
///     type: aws:codestarnotifications:NotificationRule
///     properties:
///       detailType: BASIC
///       eventTypeIds:
///         - codecommit-repository-comments-on-commits
///       name: example-code-repo-commits
///       resource: ${code.arn}
///       targets:
///         - address: ${notif.arn}
/// variables:
///   notifAccess:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sns:Publish
///             principals:
///               - type: Service
///                 identifiers:
///                   - codestar-notifications.amazonaws.com
///             resources:
///               - ${notif.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeStar notification rule using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codestarnotifications/notificationRule:NotificationRule foo arn:aws:codestar-notifications:us-west-1:0123456789:notificationrule/2cdc68a3-8f7c-4893-b6a5-45b362bd4f2b
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod notification_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationRuleArgs {
        /// The level of detail to include in the notifications for this resource. Possible values are `BASIC` and `FULL`.
        #[builder(into)]
        pub detail_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of event types associated with this notification rule.
        /// For list of allowed events see [here](https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api).
        #[builder(into)]
        pub event_type_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of notification rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the resource to associate with the notification rule.
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration blocks containing notification target information. Can be specified multiple times. At least one target must be specified on creation.
        #[builder(into, default)]
        pub targets: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::codestarnotifications::NotificationRuleTarget>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationRuleResult {
        /// The codestar notification rule ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The level of detail to include in the notifications for this resource. Possible values are `BASIC` and `FULL`.
        pub detail_type: pulumi_gestalt_rust::Output<String>,
        /// A list of event types associated with this notification rule.
        /// For list of allowed events see [here](https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api).
        pub event_type_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of notification rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the resource to associate with the notification rule.
        pub resource: pulumi_gestalt_rust::Output<String>,
        /// The status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration blocks containing notification target information. Can be specified multiple times. At least one target must be specified on creation.
        pub targets: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::codestarnotifications::NotificationRuleTarget>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NotificationRuleArgs,
    ) -> NotificationRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let detail_type_binding = args.detail_type.get_output(context);
        let event_type_ids_binding = args.event_type_ids.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_binding = args.resource.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let targets_binding = args.targets.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codestarnotifications/notificationRule:NotificationRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detailType".into(),
                    value: &detail_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventTypeIds".into(),
                    value: &event_type_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resource".into(),
                    value: &resource_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targets".into(),
                    value: &targets_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NotificationRuleResult {
            arn: o.get_field("arn"),
            detail_type: o.get_field("detailType"),
            event_type_ids: o.get_field("eventTypeIds"),
            name: o.get_field("name"),
            resource: o.get_field("resource"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            targets: o.get_field("targets"),
        }
    }
}
