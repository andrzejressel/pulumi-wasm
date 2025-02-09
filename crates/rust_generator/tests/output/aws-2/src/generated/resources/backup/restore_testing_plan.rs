/// Resource for managing an AWS Backup Restore Testing Plan.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = restore_testing_plan::create(
///         "example",
///         RestoreTestingPlanArgs::builder()
///             .recovery_point_selection(
///                 RestoreTestingPlanRecoveryPointSelection::builder()
///                     .algorithm("LATEST_WITHIN_WINDOW")
///                     .includeVaults(vec!["*",])
///                     .recoveryPointTypes(vec!["CONTINUOUS",])
///                     .build_struct(),
///             )
///             .schedule_expression("cron(0 12 ? * * *)")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Restore Testing Plan using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/restoreTestingPlan:RestoreTestingPlan example my_testing_plan
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod restore_testing_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestoreTestingPlanArgs {
        /// The name of the restore testing plan. Must be between 1 and 50 characters long and contain only alphanumeric characters and underscores.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the recovery point selection configuration. See RecoveryPointSelection section for more details.
        #[builder(into, default)]
        pub recovery_point_selection: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::RestoreTestingPlanRecoveryPointSelection>,
        >,
        /// The schedule expression for the restore testing plan.
        #[builder(into)]
        pub schedule_expression: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The timezone for the schedule expression. If not provided, the state value will be used.
        #[builder(into, default)]
        pub schedule_expression_timezone: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The number of hours in the start window for the restore testing plan. Must be between 1 and 168.
        #[builder(into, default)]
        pub start_window_hours: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RestoreTestingPlanResult {
        /// ARN of the Restore Testing Plan.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the restore testing plan. Must be between 1 and 50 characters long and contain only alphanumeric characters and underscores.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the recovery point selection configuration. See RecoveryPointSelection section for more details.
        pub recovery_point_selection: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::RestoreTestingPlanRecoveryPointSelection>,
        >,
        /// The schedule expression for the restore testing plan.
        pub schedule_expression: pulumi_gestalt_rust::Output<String>,
        /// The timezone for the schedule expression. If not provided, the state value will be used.
        pub schedule_expression_timezone: pulumi_gestalt_rust::Output<String>,
        /// The number of hours in the start window for the restore testing plan. Must be between 1 and 168.
        pub start_window_hours: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RestoreTestingPlanArgs,
    ) -> RestoreTestingPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let recovery_point_selection_binding = args
            .recovery_point_selection
            .get_output(context);
        let schedule_expression_binding = args.schedule_expression.get_output(context);
        let schedule_expression_timezone_binding = args
            .schedule_expression_timezone
            .get_output(context);
        let start_window_hours_binding = args.start_window_hours.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/restoreTestingPlan:RestoreTestingPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryPointSelection".into(),
                    value: recovery_point_selection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleExpression".into(),
                    value: schedule_expression_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleExpressionTimezone".into(),
                    value: schedule_expression_timezone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startWindowHours".into(),
                    value: start_window_hours_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RestoreTestingPlanResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            recovery_point_selection: o.get_field("recoveryPointSelection"),
            schedule_expression: o.get_field("scheduleExpression"),
            schedule_expression_timezone: o.get_field("scheduleExpressionTimezone"),
            start_window_hours: o.get_field("startWindowHours"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
