/// Provides a resource to create an EventBridge resource policy to support cross-account events.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// > **Note:** The EventBridge bus policy resource  (`aws.cloudwatch.EventBusPolicy`) is incompatible with the EventBridge permission resource (`aws.cloudwatch.EventPermission`) and will overwrite permissions.
///
/// ## Example Usage
///
/// ### Account Access
///
/// ```yaml
/// resources:
///   testEventBusPolicy:
///     type: aws:cloudwatch:EventBusPolicy
///     name: test
///     properties:
///       policy: ${test.json}
///       eventBusName: ${testAwsCloudwatchEventBus.name}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: DevAccountAccess
///             effect: Allow
///             actions:
///               - events:PutEvents
///             resources:
///               - arn:aws:events:eu-west-1:123456789012:event-bus/default
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '123456789012'
/// ```
///
/// ### Organization Access
///
/// ```yaml
/// resources:
///   testEventBusPolicy:
///     type: aws:cloudwatch:EventBusPolicy
///     name: test
///     properties:
///       policy: ${test.json}
///       eventBusName: ${testAwsCloudwatchEventBus.name}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: OrganizationAccess
///             effect: Allow
///             actions:
///               - events:DescribeRule
///               - events:ListRules
///               - events:ListTargetsByRule
///               - events:ListTagsForResource
///             resources:
///               - arn:aws:events:eu-west-1:123456789012:rule/*
///               - arn:aws:events:eu-west-1:123456789012:event-bus/default
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             conditions:
///               - test: StringEquals
///                 variable: aws:PrincipalOrgID
///                 values:
///                   - ${example.id}
/// ```
///
/// ### Multiple Statements
///
/// ```yaml
/// resources:
///   testEventBusPolicy:
///     type: aws:cloudwatch:EventBusPolicy
///     name: test
///     properties:
///       policy: ${test.json}
///       eventBusName: ${testAwsCloudwatchEventBus.name}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: DevAccountAccess
///             effect: Allow
///             actions:
///               - events:PutEvents
///             resources:
///               - arn:aws:events:eu-west-1:123456789012:event-bus/default
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '123456789012'
///           - sid: OrganizationAccess
///             effect: Allow
///             actions:
///               - events:DescribeRule
///               - events:ListRules
///               - events:ListTargetsByRule
///               - events:ListTagsForResource
///             resources:
///               - arn:aws:events:eu-west-1:123456789012:rule/*
///               - arn:aws:events:eu-west-1:123456789012:event-bus/default
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             conditions:
///               - test: StringEquals
///                 variable: aws:PrincipalOrgID
///                 values:
///                   - ${example.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an EventBridge policy using the `event_bus_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventBusPolicy:EventBusPolicy DevAccountAccess example-event-bus
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_bus_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventBusPolicyArgs {
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        #[builder(into, default)]
        pub event_bus_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The text of the policy.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EventBusPolicyResult {
        /// The name of the event bus to set the permissions on.
        /// If you omit this, the permissions are set on the `default` event bus.
        pub event_bus_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The text of the policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventBusPolicyArgs,
    ) -> EventBusPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let event_bus_name_binding = args.event_bus_name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventBusPolicy:EventBusPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventBusName".into(),
                    value: &event_bus_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventBusPolicyResult {
            event_bus_name: o.get_field("eventBusName"),
            policy: o.get_field("policy"),
        }
    }
}
