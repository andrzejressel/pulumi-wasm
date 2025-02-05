/// Resource for managing an AWS Backup Restore Testing Plan.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod restore_testing_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestoreTestingPlanArgs {
        /// The name of the restore testing plan. Must be between 1 and 50 characters long and contain only alphanumeric characters and underscores.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the recovery point selection configuration. See RecoveryPointSelection section for more details.
        #[builder(into, default)]
        pub recovery_point_selection: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::backup::RestoreTestingPlanRecoveryPointSelection>,
        >,
        /// The schedule expression for the restore testing plan.
        #[builder(into)]
        pub schedule_expression: pulumi_wasm_rust::InputOrOutput<String>,
        /// The timezone for the schedule expression. If not provided, the state value will be used.
        #[builder(into, default)]
        pub schedule_expression_timezone: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The number of hours in the start window for the restore testing plan. Must be between 1 and 168.
        #[builder(into, default)]
        pub start_window_hours: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RestoreTestingPlanResult {
        /// ARN of the Restore Testing Plan.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the restore testing plan. Must be between 1 and 50 characters long and contain only alphanumeric characters and underscores.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the recovery point selection configuration. See RecoveryPointSelection section for more details.
        pub recovery_point_selection: pulumi_wasm_rust::Output<
            Option<super::super::types::backup::RestoreTestingPlanRecoveryPointSelection>,
        >,
        /// The schedule expression for the restore testing plan.
        pub schedule_expression: pulumi_wasm_rust::Output<String>,
        /// The timezone for the schedule expression. If not provided, the state value will be used.
        pub schedule_expression_timezone: pulumi_wasm_rust::Output<String>,
        /// The number of hours in the start window for the restore testing plan. Must be between 1 and 168.
        pub start_window_hours: pulumi_wasm_rust::Output<i32>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RestoreTestingPlanArgs,
    ) -> RestoreTestingPlanResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_point_selection_binding = args
            .recovery_point_selection
            .get_output(context)
            .get_inner();
        let schedule_expression_binding = args
            .schedule_expression
            .get_output(context)
            .get_inner();
        let schedule_expression_timezone_binding = args
            .schedule_expression_timezone
            .get_output(context)
            .get_inner();
        let start_window_hours_binding = args
            .start_window_hours
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/restoreTestingPlan:RestoreTestingPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryPointSelection".into(),
                    value: &recovery_point_selection_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleExpression".into(),
                    value: &schedule_expression_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleExpressionTimezone".into(),
                    value: &schedule_expression_timezone_binding,
                },
                register_interface::ObjectField {
                    name: "startWindowHours".into(),
                    value: &start_window_hours_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RestoreTestingPlanResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            recovery_point_selection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recoveryPointSelection"),
            ),
            schedule_expression: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduleExpression"),
            ),
            schedule_expression_timezone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduleExpressionTimezone"),
            ),
            start_window_hours: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startWindowHours"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
