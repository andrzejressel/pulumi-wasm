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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScheduledActionArgs,
    ) -> ScheduledActionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let enable_binding = args.enable.get_output(context).get_inner();
        let end_time_binding = args.end_time.get_output(context).get_inner();
        let iam_role_binding = args.iam_role.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let start_time_binding = args.start_time.get_output(context).get_inner();
        let target_action_binding = args.target_action.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/scheduledAction:ScheduledAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enable".into(),
                    value: &enable_binding,
                },
                register_interface::ObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding,
                },
                register_interface::ObjectField {
                    name: "iamRole".into(),
                    value: &iam_role_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "targetAction".into(),
                    value: &target_action_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScheduledActionResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enable"),
            ),
            end_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endTime"),
            ),
            iam_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRole"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            target_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetAction"),
            ),
        }
    }
}
