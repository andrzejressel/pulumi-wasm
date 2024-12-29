/// Provides an EventBridge Target resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ### Kinesis Usage
///
/// ```yaml
/// resources:
///   yada:
///     type: aws:cloudwatch:EventTarget
///     properties:
///       targetId: Yada
///       rule: ${console.name}
///       arn: ${testStream.arn}
///       runCommandTargets:
///         - key: tag:Name
///           values:
///             - FooBar
///         - key: InstanceIds
///           values:
///             - i-162058cd308bffec2
///   console:
///     type: aws:cloudwatch:EventRule
///     properties:
///       name: capture-ec2-scaling-events
///       description: Capture all EC2 scaling events
///       eventPattern:
///         fn::toJSON:
///           source:
///             - aws.autoscaling
///           detail-type:
///             - EC2 Instance Launch Successful
///             - EC2 Instance Terminate Successful
///             - EC2 Instance Launch Unsuccessful
///             - EC2 Instance Terminate Unsuccessful
///   testStream:
///     type: aws:kinesis:Stream
///     name: test_stream
///     properties:
///       name: kinesis-test
///       shardCount: 1
/// ```
///
/// ### SSM Document Usage
///
/// ```yaml
/// resources:
///   ssmLifecycleRole:
///     type: aws:iam:Role
///     name: ssm_lifecycle
///     properties:
///       name: SSMLifecycle
///       assumeRolePolicy: ${ssmLifecycleTrust.json}
///   ssmLifecyclePolicy:
///     type: aws:iam:Policy
///     name: ssm_lifecycle
///     properties:
///       name: SSMLifecycle
///       policy: ${ssmLifecycle.json}
///   ssmLifecycleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: ssm_lifecycle
///     properties:
///       policyArn: ${ssmLifecyclePolicy.arn}
///       role: ${ssmLifecycleRole.name}
///   stopInstance:
///     type: aws:ssm:Document
///     name: stop_instance
///     properties:
///       name: stop_instance
///       documentType: Command
///       content:
///         fn::toJSON:
///           schemaVersion: '1.2'
///           description: Stop an instance
///           parameters: {}
///           runtimeConfig:
///             aws:runShellScript:
///               properties:
///                 - id: 0.aws:runShellScript
///                   runCommand:
///                     - halt
///   stopInstances:
///     type: aws:cloudwatch:EventRule
///     name: stop_instances
///     properties:
///       name: StopInstance
///       description: Stop instances nightly
///       scheduleExpression: cron(0 0 * * ? *)
///   stopInstancesEventTarget:
///     type: aws:cloudwatch:EventTarget
///     name: stop_instances
///     properties:
///       targetId: StopInstance
///       arn: ${stopInstance.arn}
///       rule: ${stopInstances.name}
///       roleArn: ${ssmLifecycleRole.arn}
///       runCommandTargets:
///         - key: tag:Terminate
///           values:
///             - midnight
/// variables:
///   ssmLifecycleTrust:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///   ssmLifecycle:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ssm:SendCommand
///             resources:
///               - arn:aws:ec2:eu-west-1:1234567890:instance/*
///             conditions:
///               - test: StringEquals
///                 variable: ec2:ResourceTag/Terminate
///                 values:
///                   - '*'
///           - effect: Allow
///             actions:
///               - ssm:SendCommand
///             resources:
///               - ${stopInstance.arn}
/// ```
///
/// ### RunCommand Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let stopInstances = event_rule::create(
///         "stopInstances",
///         EventRuleArgs::builder()
///             .description("Stop instances nightly")
///             .name("StopInstance")
///             .schedule_expression("cron(0 0 * * ? *)")
///             .build_struct(),
///     );
///     let stopInstancesEventTarget = event_target::create(
///         "stopInstancesEventTarget",
///         EventTargetArgs::builder()
///             .arn("arn:aws:ssm:${awsRegion}::document/AWS-RunShellScript")
///             .input("{\"commands\":[\"halt\"]}")
///             .role_arn("${ssmLifecycle.arn}")
///             .rule("${stopInstances.name}")
///             .run_command_targets(
///                 vec![
///                     EventTargetRunCommandTarget::builder().key("tag:Terminate")
///                     .values(vec!["midnight",]).build_struct(),
///                 ],
///             )
///             .target_id("StopInstance")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### ECS Run Task with Role and Task Override Usage
///
/// ```yaml
/// resources:
///   ecsEvents:
///     type: aws:iam:Role
///     name: ecs_events
///     properties:
///       name: ecs_events
///       assumeRolePolicy: ${assumeRole.json}
///   ecsEventsRunTaskWithAnyRoleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: ecs_events_run_task_with_any_role
///     properties:
///       name: ecs_events_run_task_with_any_role
///       role: ${ecsEvents.id}
///       policy: ${ecsEventsRunTaskWithAnyRole.json}
///   ecsScheduledTask:
///     type: aws:cloudwatch:EventTarget
///     name: ecs_scheduled_task
///     properties:
///       targetId: run-scheduled-task-every-hour
///       arn: ${clusterName.arn}
///       rule: ${everyHour.name}
///       roleArn: ${ecsEvents.arn}
///       ecsTarget:
///         taskCount: 1
///         taskDefinitionArn: ${taskName.arn}
///       input:
///         fn::toJSON:
///           containerOverrides:
///             - name: name-of-container-to-override
///               command:
///                 - bin/console
///                 - scheduled-task
/// variables:
///   assumeRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   ecsEventsRunTaskWithAnyRole:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - iam:PassRole
///             resources:
///               - '*'
///           - effect: Allow
///             actions:
///               - ecs:RunTask
///             resources:
///               - fn::invoke:
///                   Function: std:replace
///                   Arguments:
///                     text: ${taskName.arn}
///                     search: /:\d+$/
///                     replace: :*
///                   Return: result
/// ```
///
/// ### API Gateway target
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:EventTarget
///     properties:
///       arn: ${exampleStage.executionArn}/GET
///       rule: ${exampleEventRule.id}
///       httpTarget:
///         queryStringParameters:
///           Body: $.detail.body
///         headerParameters:
///           Env: Test
///   exampleEventRule:
///     type: aws:cloudwatch:EventRule
///     name: example
///   exampleDeployment:
///     type: aws:apigateway:Deployment
///     name: example
///     properties:
///       restApi: ${exampleAwsApiGatewayRestApi.id}
///   exampleStage:
///     type: aws:apigateway:Stage
///     name: example
///     properties:
///       restApi: ${exampleAwsApiGatewayRestApi.id}
///       deployment: ${exampleDeployment.id}
/// ```
///
/// ### Cross-Account Event Bus target
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["events.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let eventBusInvokeRemoteEventBus = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["events:PutEvents",]).effect("Allow")
///                     .resources(vec!["arn:aws:events:eu-west-1:1234567890:event-bus/My-Event-Bus",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let eventBusInvokeRemoteEventBusPolicy = policy::create(
///         "eventBusInvokeRemoteEventBusPolicy",
///         PolicyArgs::builder()
///             .name("event_bus_invoke_remote_event_bus")
///             .policy("${eventBusInvokeRemoteEventBus.json}")
///             .build_struct(),
///     );
///     let eventBusInvokeRemoteEventBusRole = role::create(
///         "eventBusInvokeRemoteEventBusRole",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("event-bus-invoke-remote-event-bus")
///             .build_struct(),
///     );
///     let eventBusInvokeRemoteEventBusRolePolicyAttachment = role_policy_attachment::create(
///         "eventBusInvokeRemoteEventBusRolePolicyAttachment",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn("${eventBusInvokeRemoteEventBusPolicy.arn}")
///             .role("${eventBusInvokeRemoteEventBusRole.name}")
///             .build_struct(),
///     );
///     let stopInstances = event_rule::create(
///         "stopInstances",
///         EventRuleArgs::builder()
///             .description("Stop instances nightly")
///             .name("StopInstance")
///             .schedule_expression("cron(0 0 * * ? *)")
///             .build_struct(),
///     );
///     let stopInstancesEventTarget = event_target::create(
///         "stopInstancesEventTarget",
///         EventTargetArgs::builder()
///             .arn("arn:aws:events:eu-west-1:1234567890:event-bus/My-Event-Bus")
///             .role_arn("${eventBusInvokeRemoteEventBusRole.arn}")
///             .rule("${stopInstances.name}")
///             .target_id("StopInstance")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Input Transformer Usage - JSON Object
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:EventTarget
///     properties:
///       arn: ${exampleAwsLambdaFunction.arn}
///       rule: ${exampleEventRule.id}
///       inputTransformer:
///         inputPaths:
///           instance: $.detail.instance
///           status: $.detail.status
///         inputTemplate: |
///           {
///             "instance_id": <instance>,
///             "instance_status": <status>
///           }
///   exampleEventRule:
///     type: aws:cloudwatch:EventRule
///     name: example
/// ```
///
/// ### Input Transformer Usage - Simple String
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:EventTarget
///     properties:
///       arn: ${exampleAwsLambdaFunction.arn}
///       rule: ${exampleEventRule.id}
///       inputTransformer:
///         inputPaths:
///           instance: $.detail.instance
///           status: $.detail.status
///         inputTemplate: '"<instance> is in state <status>"'
///   exampleEventRule:
///     type: aws:cloudwatch:EventRule
///     name: example
/// ```
///
/// ### Cloudwatch Log Group Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: /aws/events/guardduty/logs
///       retentionInDays: 1
///   exampleLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: example
///     properties:
///       policyDocument: ${exampleLogPolicy.json}
///       policyName: guardduty-log-publishing-policy
///   exampleEventRule:
///     type: aws:cloudwatch:EventRule
///     name: example
///     properties:
///       name: guard-duty_event_rule
///       description: GuardDuty Findings
///       eventPattern:
///         fn::toJSON:
///           source:
///             - aws.guardduty
///       tags:
///         Environment: example
///   exampleEventTarget:
///     type: aws:cloudwatch:EventTarget
///     name: example
///     properties:
///       rule: ${exampleEventRule.name}
///       arn: ${example.arn}
/// variables:
///   exampleLogPolicy:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - logs:CreateLogStream
///             resources:
///               - ${example.arn}:*
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///                   - delivery.logs.amazonaws.com
///           - effect: Allow
///             actions:
///               - logs:PutLogEvents
///             resources:
///               - ${example.arn}:*:*
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///                   - delivery.logs.amazonaws.com
///             conditions:
///               - test: ArnEquals
///                 values:
///                   - ${exampleEventRule.arn}
///                 variable: aws:SourceArn
/// ```
///
/// ### AppSync Usage
///
/// ```yaml
/// resources:
///   invokeAppsyncMutation:
///     type: aws:cloudwatch:EventRule
///     name: invoke_appsync_mutation
///     properties:
///       name: invoke-appsync-mutation
///       description: schedule_batch_test
///       scheduleExpression: rate(5 minutes)
///   invokeAppsyncMutationEventTarget:
///     type: aws:cloudwatch:EventTarget
///     name: invoke_appsync_mutation
///     properties:
///       arn:
///         fn::invoke:
///           Function: std:replace
///           Arguments:
///             text: ${["graphql-api"].arn}
///             search: apis
///             replace: endpoints/graphql-api
///           Return: result
///       rule: ${invokeAppsyncMutation.id}
///       roleArn: ${appsyncMutationRole.arn}
///       inputTransformer:
///         inputPaths:
///           input: $.detail.input
///         inputTemplate: |2
///                 {
///                   "input": <input>
///                 }
///       appsyncTarget:
///         graphqlOperation: 'mutation TestMutation($input:MutationInput!){testMutation(input: $input) {test}}'
///   appsyncMutationRole:
///     type: aws:iam:Role
///     name: appsync_mutation_role
///     properties:
///       name: appsync-mutation-role
///       assumeRolePolicy: ${appsyncMutationRoleTrust.json}
///   appsyncMutationRolePolicy:
///     type: aws:iam:Policy
///     name: appsync_mutation_role_policy
///     properties:
///       name: appsync-mutation-role-policy
///       policy: ${appsyncMutationRolePolicyDocument.json}
///   appsyncMutationRoleAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: appsync_mutation_role_attachment
///     properties:
///       policyArn: ${appsyncMutationRolePolicy.arn}
///       role: ${appsyncMutationRole.name}
///   graphql-api:
///     type: aws:appsync:GraphQLApi
///     properties:
///       name: api
///       authenticationType: AWS_IAM
///       schema: |2
///             schema {
///               mutation: Mutation
///               query: Query
///             }
///
///             type Query {
///               testQuery: String
///             }
///
///             type Mutation {
///               testMutation(input: MutationInput!): TestMutationResult
///             }
///
///             type TestMutationResult {
///               test: String
///             }
///
///             input MutationInput {
///               testInput: String
///             }
/// variables:
///   appsyncMutationRoleTrust:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///   appsyncMutationRolePolicyDocument:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - appsync:GraphQL
///             effect: Allow
///             resources:
///               - ${["graphql-api"].arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge Targets using `event_bus_name/rule-name/target-id` (if you omit `event_bus_name`, the `default` event bus will be used). For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventTarget:EventTarget test-event-target rule-name/target-id
/// ```
pub mod event_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventTargetArgs {
        /// Parameters used when you are using the rule to invoke an AppSync GraphQL API mutation. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub appsync_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetAppsyncTarget>,
        >,
        /// The Amazon Resource Name (ARN) of the target.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Parameters used when you are using the rule to invoke an Amazon Batch Job. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub batch_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetBatchTarget>,
        >,
        /// Parameters used when you are providing a dead letter config. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub dead_letter_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetDeadLetterConfig>,
        >,
        /// Parameters used when you are using the rule to invoke Amazon ECS Task. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub ecs_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetEcsTarget>,
        >,
        /// The name or ARN of the event bus to associate with the rule.
        /// If you omit this, the `default` event bus is used.
        #[builder(into, default)]
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to delete managed rules created by AWS. Defaults to `false`.
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Parameters used when you are using the rule to invoke an API Gateway REST endpoint. Documented below. A maximum of 1 is allowed.
        #[builder(into, default)]
        pub http_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetHttpTarget>,
        >,
        /// Valid JSON text passed to the target. Conflicts with `input_path` and `input_transformer`.
        #[builder(into, default)]
        pub input: pulumi_wasm_rust::Output<Option<String>>,
        /// The value of the [JSONPath](http://goessner.net/articles/JsonPath/) that is used for extracting part of the matched event when passing it to the target. Conflicts with `input` and `input_transformer`.
        #[builder(into, default)]
        pub input_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Parameters used when you are providing a custom input to a target based on certain event data. Conflicts with `input` and `input_path`.
        #[builder(into, default)]
        pub input_transformer: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetInputTransformer>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Kinesis Stream. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub kinesis_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetKinesisTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Redshift Statement. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub redshift_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetRedshiftTarget>,
        >,
        /// Parameters used when you are providing retry policies. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub retry_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetRetryPolicy>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. Required if `ecs_target` is used or target in `arn` is EC2 instance, Kinesis data stream, Step Functions state machine, or Event Bus in different account or region.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the rule you want to add targets to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub rule: pulumi_wasm_rust::Output<String>,
        /// Parameters used when you are using the rule to invoke Amazon EC2 Run Command. Documented below. A maximum of 5 are allowed.
        #[builder(into, default)]
        pub run_command_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::EventTargetRunCommandTarget>>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SageMaker Pipeline. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub sagemaker_pipeline_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetSagemakerPipelineTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SQS Queue. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub sqs_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetSqsTarget>,
        >,
        /// The unique target assignment ID. If missing, will generate a random, unique id.
        #[builder(into, default)]
        pub target_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventTargetResult {
        /// Parameters used when you are using the rule to invoke an AppSync GraphQL API mutation. Documented below. A maximum of 1 are allowed.
        pub appsync_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetAppsyncTarget>,
        >,
        /// The Amazon Resource Name (ARN) of the target.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Parameters used when you are using the rule to invoke an Amazon Batch Job. Documented below. A maximum of 1 are allowed.
        pub batch_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetBatchTarget>,
        >,
        /// Parameters used when you are providing a dead letter config. Documented below. A maximum of 1 are allowed.
        pub dead_letter_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetDeadLetterConfig>,
        >,
        /// Parameters used when you are using the rule to invoke Amazon ECS Task. Documented below. A maximum of 1 are allowed.
        pub ecs_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetEcsTarget>,
        >,
        /// The name or ARN of the event bus to associate with the rule.
        /// If you omit this, the `default` event bus is used.
        pub event_bus_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to delete managed rules created by AWS. Defaults to `false`.
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Parameters used when you are using the rule to invoke an API Gateway REST endpoint. Documented below. A maximum of 1 is allowed.
        pub http_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetHttpTarget>,
        >,
        /// Valid JSON text passed to the target. Conflicts with `input_path` and `input_transformer`.
        pub input: pulumi_wasm_rust::Output<Option<String>>,
        /// The value of the [JSONPath](http://goessner.net/articles/JsonPath/) that is used for extracting part of the matched event when passing it to the target. Conflicts with `input` and `input_transformer`.
        pub input_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Parameters used when you are providing a custom input to a target based on certain event data. Conflicts with `input` and `input_path`.
        pub input_transformer: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetInputTransformer>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Kinesis Stream. Documented below. A maximum of 1 are allowed.
        pub kinesis_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetKinesisTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Redshift Statement. Documented below. A maximum of 1 are allowed.
        pub redshift_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetRedshiftTarget>,
        >,
        /// Parameters used when you are providing retry policies. Documented below. A maximum of 1 are allowed.
        pub retry_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetRetryPolicy>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. Required if `ecs_target` is used or target in `arn` is EC2 instance, Kinesis data stream, Step Functions state machine, or Event Bus in different account or region.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the rule you want to add targets to.
        ///
        /// The following arguments are optional:
        pub rule: pulumi_wasm_rust::Output<String>,
        /// Parameters used when you are using the rule to invoke Amazon EC2 Run Command. Documented below. A maximum of 5 are allowed.
        pub run_command_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cloudwatch::EventTargetRunCommandTarget>>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SageMaker Pipeline. Documented below. A maximum of 1 are allowed.
        pub sagemaker_pipeline_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetSagemakerPipelineTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SQS Queue. Documented below. A maximum of 1 are allowed.
        pub sqs_target: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetSqsTarget>,
        >,
        /// The unique target assignment ID. If missing, will generate a random, unique id.
        pub target_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventTargetArgs) -> EventTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let appsync_target_binding = args.appsync_target.get_inner();
        let arn_binding = args.arn.get_inner();
        let batch_target_binding = args.batch_target.get_inner();
        let dead_letter_config_binding = args.dead_letter_config.get_inner();
        let ecs_target_binding = args.ecs_target.get_inner();
        let event_bus_name_binding = args.event_bus_name.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let http_target_binding = args.http_target.get_inner();
        let input_binding = args.input.get_inner();
        let input_path_binding = args.input_path.get_inner();
        let input_transformer_binding = args.input_transformer.get_inner();
        let kinesis_target_binding = args.kinesis_target.get_inner();
        let redshift_target_binding = args.redshift_target.get_inner();
        let retry_policy_binding = args.retry_policy.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let rule_binding = args.rule.get_inner();
        let run_command_targets_binding = args.run_command_targets.get_inner();
        let sagemaker_pipeline_target_binding = args
            .sagemaker_pipeline_target
            .get_inner();
        let sqs_target_binding = args.sqs_target.get_inner();
        let target_id_binding = args.target_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventTarget:EventTarget".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appsyncTarget".into(),
                    value: &appsync_target_binding,
                },
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "batchTarget".into(),
                    value: &batch_target_binding,
                },
                register_interface::ObjectField {
                    name: "deadLetterConfig".into(),
                    value: &dead_letter_config_binding,
                },
                register_interface::ObjectField {
                    name: "ecsTarget".into(),
                    value: &ecs_target_binding,
                },
                register_interface::ObjectField {
                    name: "eventBusName".into(),
                    value: &event_bus_name_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "httpTarget".into(),
                    value: &http_target_binding,
                },
                register_interface::ObjectField {
                    name: "input".into(),
                    value: &input_binding,
                },
                register_interface::ObjectField {
                    name: "inputPath".into(),
                    value: &input_path_binding,
                },
                register_interface::ObjectField {
                    name: "inputTransformer".into(),
                    value: &input_transformer_binding,
                },
                register_interface::ObjectField {
                    name: "kinesisTarget".into(),
                    value: &kinesis_target_binding,
                },
                register_interface::ObjectField {
                    name: "redshiftTarget".into(),
                    value: &redshift_target_binding,
                },
                register_interface::ObjectField {
                    name: "retryPolicy".into(),
                    value: &retry_policy_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "rule".into(),
                    value: &rule_binding,
                },
                register_interface::ObjectField {
                    name: "runCommandTargets".into(),
                    value: &run_command_targets_binding,
                },
                register_interface::ObjectField {
                    name: "sagemakerPipelineTarget".into(),
                    value: &sagemaker_pipeline_target_binding,
                },
                register_interface::ObjectField {
                    name: "sqsTarget".into(),
                    value: &sqs_target_binding,
                },
                register_interface::ObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appsyncTarget".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "batchTarget".into(),
                },
                register_interface::ResultField {
                    name: "deadLetterConfig".into(),
                },
                register_interface::ResultField {
                    name: "ecsTarget".into(),
                },
                register_interface::ResultField {
                    name: "eventBusName".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "httpTarget".into(),
                },
                register_interface::ResultField {
                    name: "input".into(),
                },
                register_interface::ResultField {
                    name: "inputPath".into(),
                },
                register_interface::ResultField {
                    name: "inputTransformer".into(),
                },
                register_interface::ResultField {
                    name: "kinesisTarget".into(),
                },
                register_interface::ResultField {
                    name: "redshiftTarget".into(),
                },
                register_interface::ResultField {
                    name: "retryPolicy".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "rule".into(),
                },
                register_interface::ResultField {
                    name: "runCommandTargets".into(),
                },
                register_interface::ResultField {
                    name: "sagemakerPipelineTarget".into(),
                },
                register_interface::ResultField {
                    name: "sqsTarget".into(),
                },
                register_interface::ResultField {
                    name: "targetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventTargetResult {
            appsync_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appsyncTarget").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            batch_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchTarget").unwrap(),
            ),
            dead_letter_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deadLetterConfig").unwrap(),
            ),
            ecs_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecsTarget").unwrap(),
            ),
            event_bus_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventBusName").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            http_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpTarget").unwrap(),
            ),
            input: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("input").unwrap(),
            ),
            input_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputPath").unwrap(),
            ),
            input_transformer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputTransformer").unwrap(),
            ),
            kinesis_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kinesisTarget").unwrap(),
            ),
            redshift_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redshiftTarget").unwrap(),
            ),
            retry_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryPolicy").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rule").unwrap(),
            ),
            run_command_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runCommandTargets").unwrap(),
            ),
            sagemaker_pipeline_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sagemakerPipelineTarget").unwrap(),
            ),
            sqs_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqsTarget").unwrap(),
            ),
            target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetId").unwrap(),
            ),
        }
    }
}
