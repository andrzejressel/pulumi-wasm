/// ## Example Usage
///
/// ### Pause Cluster Action
///
/// ```yaml
/// resources:
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: redshift_scheduled_action
///       assumeRolePolicy: ${assumeRole.json}
///   examplePolicy:
///     type: aws:iam:Policy
///     name: example
///     properties:
///       name: redshift_scheduled_action
///       policy: ${example.json}
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       policyArn: ${examplePolicy.arn}
///       role: ${exampleRole.name}
///   exampleScheduledAction:
///     type: aws:redshift:ScheduledAction
///     name: example
///     properties:
///       name: tf-redshift-scheduled-action
///       schedule: cron(00 23 * * ? *)
///       iamRole: ${exampleRole.arn}
///       targetAction:
///         pauseCluster:
///           clusterIdentifier: tf-redshift001
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - scheduler.redshift.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - redshift:PauseCluster
///               - redshift:ResumeCluster
///               - redshift:ResizeCluster
///             resources:
///               - '*'
/// ```
///
/// ### Resize Cluster Action
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = scheduled_action::create(
///         "example",
///         ScheduledActionArgs::builder()
///             .iam_role("${exampleAwsIamRole.arn}")
///             .name("tf-redshift-scheduled-action")
///             .schedule("cron(00 23 * * ? *)")
///             .target_action(
///                 ScheduledActionTargetAction::builder()
///                     .resizeCluster(
///                         ScheduledActionTargetActionResizeCluster::builder()
///                             .clusterIdentifier("tf-redshift001")
///                             .clusterType("multi-node")
///                             .nodeType("dc1.large")
///                             .numberOfNodes(2)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Scheduled Action using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/scheduledAction:ScheduledAction example tf-redshift-scheduled-action
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scheduled_action {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScheduledActionArgs {
        /// The description of the scheduled action.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable the scheduled action. Default is `true` .
        #[builder(into, default)]
        pub enable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The end time in UTC when the schedule is active, in UTC RFC3339 format(for example, YYYY-MM-DDTHH:MM:SSZ).
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM role to assume to run the scheduled action.
        #[builder(into)]
        pub iam_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The scheduled action name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The schedule of action. The schedule is defined format of "at expression" or "cron expression", for example `at(2016-03-04T17:27:00)` or `cron(0 10 ? * MON *)`. See [Scheduled Action](https://docs.aws.amazon.com/redshift/latest/APIReference/API_ScheduledAction.html) for more information.
        #[builder(into)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The start time in UTC when the schedule is active, in UTC RFC3339 format(for example, YYYY-MM-DDTHH:MM:SSZ).
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Target action. Documented below.
        #[builder(into)]
        pub target_action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::redshift::ScheduledActionTargetAction,
        >,
    }
    #[allow(dead_code)]
    pub struct ScheduledActionResult {
        /// The description of the scheduled action.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to enable the scheduled action. Default is `true` .
        pub enable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The end time in UTC when the schedule is active, in UTC RFC3339 format(for example, YYYY-MM-DDTHH:MM:SSZ).
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IAM role to assume to run the scheduled action.
        pub iam_role: pulumi_gestalt_rust::Output<String>,
        /// The scheduled action name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The schedule of action. The schedule is defined format of "at expression" or "cron expression", for example `at(2016-03-04T17:27:00)` or `cron(0 10 ? * MON *)`. See [Scheduled Action](https://docs.aws.amazon.com/redshift/latest/APIReference/API_ScheduledAction.html) for more information.
        pub schedule: pulumi_gestalt_rust::Output<String>,
        /// The start time in UTC when the schedule is active, in UTC RFC3339 format(for example, YYYY-MM-DDTHH:MM:SSZ).
        pub start_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Target action. Documented below.
        pub target_action: pulumi_gestalt_rust::Output<
            super::super::types::redshift::ScheduledActionTargetAction,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScheduledActionArgs,
    ) -> ScheduledActionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let enable_binding = args.enable.get_output(context);
        let end_time_binding = args.end_time.get_output(context);
        let iam_role_binding = args.iam_role.get_output(context);
        let name_binding = args.name.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let start_time_binding = args.start_time.get_output(context);
        let target_action_binding = args.target_action.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/scheduledAction:ScheduledAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enable".into(),
                    value: &enable_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRole".into(),
                    value: &iam_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetAction".into(),
                    value: &target_action_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScheduledActionResult {
            description: o.get_field("description"),
            enable: o.get_field("enable"),
            end_time: o.get_field("endTime"),
            iam_role: o.get_field("iamRole"),
            name: o.get_field("name"),
            schedule: o.get_field("schedule"),
            start_time: o.get_field("startTime"),
            target_action: o.get_field("targetAction"),
        }
    }
}
