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
pub mod notification_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NotificationRuleArgs {
        /// The level of detail to include in the notifications for this resource. Possible values are `BASIC` and `FULL`.
        #[builder(into)]
        pub detail_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of event types associated with this notification rule.
        /// For list of allowed events see [here](https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api).
        #[builder(into)]
        pub event_type_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The name of notification rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARN of the resource to associate with the notification rule.
        #[builder(into)]
        pub resource: pulumi_wasm_rust::InputOrOutput<String>,
        /// The status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration blocks containing notification target information. Can be specified multiple times. At least one target must be specified on creation.
        #[builder(into, default)]
        pub targets: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::codestarnotifications::NotificationRuleTarget>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct NotificationRuleResult {
        /// The codestar notification rule ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The level of detail to include in the notifications for this resource. Possible values are `BASIC` and `FULL`.
        pub detail_type: pulumi_wasm_rust::Output<String>,
        /// A list of event types associated with this notification rule.
        /// For list of allowed events see [here](https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api).
        pub event_type_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of notification rule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the resource to associate with the notification rule.
        pub resource: pulumi_wasm_rust::Output<String>,
        /// The status of the notification rule. Possible values are `ENABLED` and `DISABLED`, default is `ENABLED`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration blocks containing notification target information. Can be specified multiple times. At least one target must be specified on creation.
        pub targets: pulumi_wasm_rust::Output<
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NotificationRuleArgs,
    ) -> NotificationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let detail_type_binding = args.detail_type.get_output(context).get_inner();
        let event_type_ids_binding = args.event_type_ids.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_binding = args.resource.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let targets_binding = args.targets.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codestarnotifications/notificationRule:NotificationRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "detailType".into(),
                    value: &detail_type_binding,
                },
                register_interface::ObjectField {
                    name: "eventTypeIds".into(),
                    value: &event_type_ids_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resource".into(),
                    value: &resource_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targets".into(),
                    value: &targets_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NotificationRuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            detail_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("detailType"),
            ),
            event_type_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventTypeIds"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resource"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            targets: pulumi_wasm_rust::__private::into_domain(o.extract_field("targets")),
        }
    }
}
