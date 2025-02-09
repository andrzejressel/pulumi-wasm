/// Provides an SSM Maintenance Window Task resource
///
/// ## Example Usage
///
/// ### Automation Tasks
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssm:MaintenanceWindowTask
///     properties:
///       maxConcurrency: 2
///       maxErrors: 1
///       priority: 1
///       taskArn: AWS-RestartEC2Instance
///       taskType: AUTOMATION
///       windowId: ${exampleAwsSsmMaintenanceWindow.id}
///       targets:
///         - key: InstanceIds
///           values:
///             - ${exampleAwsInstance.id}
///       taskInvocationParameters:
///         automationParameters:
///           documentVersion: $LATEST
///           parameters:
///             - name: InstanceId
///               values:
///                 - ${exampleAwsInstance.id}
/// ```
///
/// ### Lambda Tasks
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssm:MaintenanceWindowTask
///     properties:
///       maxConcurrency: 2
///       maxErrors: 1
///       priority: 1
///       taskArn: ${exampleAwsLambdaFunction.arn}
///       taskType: LAMBDA
///       windowId: ${exampleAwsSsmMaintenanceWindow.id}
///       targets:
///         - key: InstanceIds
///           values:
///             - ${exampleAwsInstance.id}
///       taskInvocationParameters:
///         lambdaParameters:
///           clientContext:
///             fn::invoke:
///               function: std:base64encode
///               arguments:
///                 input: '{"key1":"value1"}'
///               return: result
///           payload: '{"key1":"value1"}'
/// ```
///
/// ### Run Command Tasks
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssm:MaintenanceWindowTask
///     properties:
///       maxConcurrency: 2
///       maxErrors: 1
///       priority: 1
///       taskArn: AWS-RunShellScript
///       taskType: RUN_COMMAND
///       windowId: ${exampleAwsSsmMaintenanceWindow.id}
///       targets:
///         - key: InstanceIds
///           values:
///             - ${exampleAwsInstance.id}
///       taskInvocationParameters:
///         runCommandParameters:
///           outputS3Bucket: ${exampleAwsS3Bucket.id}
///           outputS3KeyPrefix: output
///           serviceRoleArn: ${exampleAwsIamRole.arn}
///           timeoutSeconds: 600
///           notificationConfig:
///             notificationArn: ${exampleAwsSnsTopic.arn}
///             notificationEvents:
///               - All
///             notificationType: Command
///           parameters:
///             - name: commands
///               values:
///                 - date
/// ```
///
/// ### Step Function Tasks
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ssm:MaintenanceWindowTask
///     properties:
///       maxConcurrency: 2
///       maxErrors: 1
///       priority: 1
///       taskArn: ${exampleAwsSfnActivity.id}
///       taskType: STEP_FUNCTIONS
///       windowId: ${exampleAwsSsmMaintenanceWindow.id}
///       targets:
///         - key: InstanceIds
///           values:
///             - ${exampleAwsInstance.id}
///       taskInvocationParameters:
///         stepFunctionsParameters:
///           input: '{"key1":"value1"}'
///           name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Maintenance Window Task using the `window_id` and `window_task_id` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/maintenanceWindowTask:MaintenanceWindowTask task <window_id>/<window_task_id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod maintenance_window_task {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MaintenanceWindowTaskArgs {
        /// Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached. Valid values are `CONTINUE_TASK` and `CANCEL_TASK`.
        #[builder(into, default)]
        pub cutoff_behavior: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the maintenance window task.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of targets this task can be run for in parallel.
        #[builder(into, default)]
        pub max_concurrency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of errors allowed before this task stops being scheduled.
        #[builder(into, default)]
        pub max_errors: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the maintenance window task.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The role that should be assumed when executing the task. If a role is not provided, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created for you.
        #[builder(into, default)]
        pub service_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The targets (either instances or window target ids). Instances are specified using Key=InstanceIds,Values=instanceid1,instanceid2. Window target ids are specified using Key=WindowTargetIds,Values=window target id1, window target id2.
        #[builder(into, default)]
        pub targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTarget>>,
        >,
        /// The ARN of the task to execute.
        #[builder(into)]
        pub task_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block with parameters for task execution.
        #[builder(into, default)]
        pub task_invocation_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParameters,
            >,
        >,
        /// The type of task being registered. Valid values: `AUTOMATION`, `LAMBDA`, `RUN_COMMAND` or `STEP_FUNCTIONS`.
        #[builder(into)]
        pub task_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Id of the maintenance window to register the task with.
        #[builder(into)]
        pub window_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MaintenanceWindowTaskResult {
        /// The ARN of the maintenance window task.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached. Valid values are `CONTINUE_TASK` and `CANCEL_TASK`.
        pub cutoff_behavior: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the maintenance window task.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum number of targets this task can be run for in parallel.
        pub max_concurrency: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of errors allowed before this task stops being scheduled.
        pub max_errors: pulumi_gestalt_rust::Output<String>,
        /// The name of the maintenance window task.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The role that should be assumed when executing the task. If a role is not provided, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created for you.
        pub service_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The targets (either instances or window target ids). Instances are specified using Key=InstanceIds,Values=instanceid1,instanceid2. Window target ids are specified using Key=WindowTargetIds,Values=window target id1, window target id2.
        pub targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTarget>>,
        >,
        /// The ARN of the task to execute.
        pub task_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with parameters for task execution.
        pub task_invocation_parameters: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParameters,
            >,
        >,
        /// The type of task being registered. Valid values: `AUTOMATION`, `LAMBDA`, `RUN_COMMAND` or `STEP_FUNCTIONS`.
        pub task_type: pulumi_gestalt_rust::Output<String>,
        /// The Id of the maintenance window to register the task with.
        pub window_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the maintenance window task.
        pub window_task_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MaintenanceWindowTaskArgs,
    ) -> MaintenanceWindowTaskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cutoff_behavior_binding = args.cutoff_behavior.get_output(context);
        let description_binding = args.description.get_output(context);
        let max_concurrency_binding = args.max_concurrency.get_output(context);
        let max_errors_binding = args.max_errors.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let service_role_arn_binding = args.service_role_arn.get_output(context);
        let targets_binding = args.targets.get_output(context);
        let task_arn_binding = args.task_arn.get_output(context);
        let task_invocation_parameters_binding = args
            .task_invocation_parameters
            .get_output(context);
        let task_type_binding = args.task_type.get_output(context);
        let window_id_binding = args.window_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/maintenanceWindowTask:MaintenanceWindowTask".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cutoffBehavior".into(),
                    value: cutoff_behavior_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxConcurrency".into(),
                    value: max_concurrency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxErrors".into(),
                    value: max_errors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceRoleArn".into(),
                    value: service_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targets".into(),
                    value: targets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskArn".into(),
                    value: task_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskInvocationParameters".into(),
                    value: task_invocation_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskType".into(),
                    value: task_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "windowId".into(),
                    value: window_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MaintenanceWindowTaskResult {
            arn: o.get_field("arn"),
            cutoff_behavior: o.get_field("cutoffBehavior"),
            description: o.get_field("description"),
            max_concurrency: o.get_field("maxConcurrency"),
            max_errors: o.get_field("maxErrors"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            service_role_arn: o.get_field("serviceRoleArn"),
            targets: o.get_field("targets"),
            task_arn: o.get_field("taskArn"),
            task_invocation_parameters: o.get_field("taskInvocationParameters"),
            task_type: o.get_field("taskType"),
            window_id: o.get_field("windowId"),
            window_task_id: o.get_field("windowTaskId"),
        }
    }
}
