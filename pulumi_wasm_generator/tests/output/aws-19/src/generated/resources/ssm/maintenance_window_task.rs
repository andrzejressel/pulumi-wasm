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
pub mod maintenance_window_task {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MaintenanceWindowTaskArgs {
        /// Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached. Valid values are `CONTINUE_TASK` and `CANCEL_TASK`.
        #[builder(into, default)]
        pub cutoff_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the maintenance window task.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of targets this task can be run for in parallel.
        #[builder(into, default)]
        pub max_concurrency: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of errors allowed before this task stops being scheduled.
        #[builder(into, default)]
        pub max_errors: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the maintenance window task.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The role that should be assumed when executing the task. If a role is not provided, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created for you.
        #[builder(into, default)]
        pub service_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The targets (either instances or window target ids). Instances are specified using Key=InstanceIds,Values=instanceid1,instanceid2. Window target ids are specified using Key=WindowTargetIds,Values=window target id1, window target id2.
        #[builder(into, default)]
        pub targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTarget>>,
        >,
        /// The ARN of the task to execute.
        #[builder(into)]
        pub task_arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block with parameters for task execution.
        #[builder(into, default)]
        pub task_invocation_parameters: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParameters,
            >,
        >,
        /// The type of task being registered. Valid values: `AUTOMATION`, `LAMBDA`, `RUN_COMMAND` or `STEP_FUNCTIONS`.
        #[builder(into)]
        pub task_type: pulumi_wasm_rust::Output<String>,
        /// The Id of the maintenance window to register the task with.
        #[builder(into)]
        pub window_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MaintenanceWindowTaskResult {
        /// The ARN of the maintenance window task.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Indicates whether tasks should continue to run after the cutoff time specified in the maintenance windows is reached. Valid values are `CONTINUE_TASK` and `CANCEL_TASK`.
        pub cutoff_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the maintenance window task.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum number of targets this task can be run for in parallel.
        pub max_concurrency: pulumi_wasm_rust::Output<String>,
        /// The maximum number of errors allowed before this task stops being scheduled.
        pub max_errors: pulumi_wasm_rust::Output<String>,
        /// The name of the maintenance window task.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The role that should be assumed when executing the task. If a role is not provided, Systems Manager uses your account's service-linked role. If no service-linked role for Systems Manager exists in your account, it is created for you.
        pub service_role_arn: pulumi_wasm_rust::Output<String>,
        /// The targets (either instances or window target ids). Instances are specified using Key=InstanceIds,Values=instanceid1,instanceid2. Window target ids are specified using Key=WindowTargetIds,Values=window target id1, window target id2.
        pub targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTarget>>,
        >,
        /// The ARN of the task to execute.
        pub task_arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block with parameters for task execution.
        pub task_invocation_parameters: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParameters,
            >,
        >,
        /// The type of task being registered. Valid values: `AUTOMATION`, `LAMBDA`, `RUN_COMMAND` or `STEP_FUNCTIONS`.
        pub task_type: pulumi_wasm_rust::Output<String>,
        /// The Id of the maintenance window to register the task with.
        pub window_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the maintenance window task.
        pub window_task_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MaintenanceWindowTaskArgs,
    ) -> MaintenanceWindowTaskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cutoff_behavior_binding = args.cutoff_behavior.get_inner();
        let description_binding = args.description.get_inner();
        let max_concurrency_binding = args.max_concurrency.get_inner();
        let max_errors_binding = args.max_errors.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let service_role_arn_binding = args.service_role_arn.get_inner();
        let targets_binding = args.targets.get_inner();
        let task_arn_binding = args.task_arn.get_inner();
        let task_invocation_parameters_binding = args
            .task_invocation_parameters
            .get_inner();
        let task_type_binding = args.task_type.get_inner();
        let window_id_binding = args.window_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/maintenanceWindowTask:MaintenanceWindowTask".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cutoffBehavior".into(),
                    value: &cutoff_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "maxConcurrency".into(),
                    value: &max_concurrency_binding,
                },
                register_interface::ObjectField {
                    name: "maxErrors".into(),
                    value: &max_errors_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRoleArn".into(),
                    value: &service_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "targets".into(),
                    value: &targets_binding,
                },
                register_interface::ObjectField {
                    name: "taskArn".into(),
                    value: &task_arn_binding,
                },
                register_interface::ObjectField {
                    name: "taskInvocationParameters".into(),
                    value: &task_invocation_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "taskType".into(),
                    value: &task_type_binding,
                },
                register_interface::ObjectField {
                    name: "windowId".into(),
                    value: &window_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cutoffBehavior".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "maxConcurrency".into(),
                },
                register_interface::ResultField {
                    name: "maxErrors".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "serviceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "targets".into(),
                },
                register_interface::ResultField {
                    name: "taskArn".into(),
                },
                register_interface::ResultField {
                    name: "taskInvocationParameters".into(),
                },
                register_interface::ResultField {
                    name: "taskType".into(),
                },
                register_interface::ResultField {
                    name: "windowId".into(),
                },
                register_interface::ResultField {
                    name: "windowTaskId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MaintenanceWindowTaskResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cutoff_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cutoffBehavior").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            max_concurrency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxConcurrency").unwrap(),
            ),
            max_errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxErrors").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRoleArn").unwrap(),
            ),
            targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targets").unwrap(),
            ),
            task_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskArn").unwrap(),
            ),
            task_invocation_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskInvocationParameters").unwrap(),
            ),
            task_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskType").unwrap(),
            ),
            window_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windowId").unwrap(),
            ),
            window_task_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windowTaskId").unwrap(),
            ),
        }
    }
}
