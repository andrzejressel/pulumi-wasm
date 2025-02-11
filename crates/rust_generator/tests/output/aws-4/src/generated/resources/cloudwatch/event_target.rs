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
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///   ssmLifecycle:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///                   function: std:replace
///                   arguments:
///                     text: ${taskName.arn}
///                     search: /:\d+$/
///                     replace: :*
///                   return: result
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
/// ```yaml
/// resources:
///   eventBusInvokeRemoteEventBusRole:
///     type: aws:iam:Role
///     name: event_bus_invoke_remote_event_bus
///     properties:
///       name: event-bus-invoke-remote-event-bus
///       assumeRolePolicy: ${assumeRole.json}
///   eventBusInvokeRemoteEventBusPolicy:
///     type: aws:iam:Policy
///     name: event_bus_invoke_remote_event_bus
///     properties:
///       name: event_bus_invoke_remote_event_bus
///       policy: ${eventBusInvokeRemoteEventBus.json}
///   eventBusInvokeRemoteEventBusRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: event_bus_invoke_remote_event_bus
///     properties:
///       role: ${eventBusInvokeRemoteEventBusRole.name}
///       policyArn: ${eventBusInvokeRemoteEventBusPolicy.arn}
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
///       arn: arn:aws:events:eu-west-1:1234567890:event-bus/My-Event-Bus
///       rule: ${stopInstances.name}
///       roleArn: ${eventBusInvokeRemoteEventBusRole.arn}
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
///                   - events.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   eventBusInvokeRemoteEventBus:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - events:PutEvents
///             resources:
///               - arn:aws:events:eu-west-1:1234567890:event-bus/My-Event-Bus
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
///       function: aws:iam:getPolicyDocument
///       arguments:
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
///           function: std:replace
///           arguments:
///             text: ${["graphql-api"].arn}
///             search: apis
///             replace: endpoints/graphql-api
///           return: result
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
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - events.amazonaws.com
///   appsyncMutationRolePolicyDocument:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventTargetArgs {
        /// Parameters used when you are using the rule to invoke an AppSync GraphQL API mutation. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub appsync_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetAppsyncTarget>,
        >,
        /// The Amazon Resource Name (ARN) of the target.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Parameters used when you are using the rule to invoke an Amazon Batch Job. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub batch_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetBatchTarget>,
        >,
        /// Parameters used when you are providing a dead letter config. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub dead_letter_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetDeadLetterConfig>,
        >,
        /// Parameters used when you are using the rule to invoke Amazon ECS Task. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub ecs_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetEcsTarget>,
        >,
        /// The name or ARN of the event bus to associate with the rule.
        /// If you omit this, the `default` event bus is used.
        #[builder(into, default)]
        pub event_bus_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to delete managed rules created by AWS. Defaults to `false`.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Parameters used when you are using the rule to invoke an API Gateway REST endpoint. Documented below. A maximum of 1 is allowed.
        #[builder(into, default)]
        pub http_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetHttpTarget>,
        >,
        /// Valid JSON text passed to the target. Conflicts with `input_path` and `input_transformer`.
        #[builder(into, default)]
        pub input: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the [JSONPath](http://goessner.net/articles/JsonPath/) that is used for extracting part of the matched event when passing it to the target. Conflicts with `input` and `input_transformer`.
        #[builder(into, default)]
        pub input_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters used when you are providing a custom input to a target based on certain event data. Conflicts with `input` and `input_path`.
        #[builder(into, default)]
        pub input_transformer: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetInputTransformer>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Kinesis Stream. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub kinesis_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetKinesisTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Redshift Statement. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub redshift_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetRedshiftTarget>,
        >,
        /// Parameters used when you are providing retry policies. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub retry_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetRetryPolicy>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. Required if `ecs_target` is used or target in `arn` is EC2 instance, Kinesis data stream, Step Functions state machine, or Event Bus in different account or region.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the rule you want to add targets to.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub rule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Parameters used when you are using the rule to invoke Amazon EC2 Run Command. Documented below. A maximum of 5 are allowed.
        #[builder(into, default)]
        pub run_command_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudwatch::EventTargetRunCommandTarget>>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SageMaker Pipeline. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub sagemaker_pipeline_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetSagemakerPipelineTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SQS Queue. Documented below. A maximum of 1 are allowed.
        #[builder(into, default)]
        pub sqs_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudwatch::EventTargetSqsTarget>,
        >,
        /// The unique target assignment ID. If missing, will generate a random, unique id.
        #[builder(into, default)]
        pub target_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventTargetResult {
        /// Parameters used when you are using the rule to invoke an AppSync GraphQL API mutation. Documented below. A maximum of 1 are allowed.
        pub appsync_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetAppsyncTarget>,
        >,
        /// The Amazon Resource Name (ARN) of the target.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Parameters used when you are using the rule to invoke an Amazon Batch Job. Documented below. A maximum of 1 are allowed.
        pub batch_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetBatchTarget>,
        >,
        /// Parameters used when you are providing a dead letter config. Documented below. A maximum of 1 are allowed.
        pub dead_letter_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetDeadLetterConfig>,
        >,
        /// Parameters used when you are using the rule to invoke Amazon ECS Task. Documented below. A maximum of 1 are allowed.
        pub ecs_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetEcsTarget>,
        >,
        /// The name or ARN of the event bus to associate with the rule.
        /// If you omit this, the `default` event bus is used.
        pub event_bus_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Used to delete managed rules created by AWS. Defaults to `false`.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Parameters used when you are using the rule to invoke an API Gateway REST endpoint. Documented below. A maximum of 1 is allowed.
        pub http_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetHttpTarget>,
        >,
        /// Valid JSON text passed to the target. Conflicts with `input_path` and `input_transformer`.
        pub input: pulumi_gestalt_rust::Output<Option<String>>,
        /// The value of the [JSONPath](http://goessner.net/articles/JsonPath/) that is used for extracting part of the matched event when passing it to the target. Conflicts with `input` and `input_transformer`.
        pub input_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// Parameters used when you are providing a custom input to a target based on certain event data. Conflicts with `input` and `input_path`.
        pub input_transformer: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetInputTransformer>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Kinesis Stream. Documented below. A maximum of 1 are allowed.
        pub kinesis_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetKinesisTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon Redshift Statement. Documented below. A maximum of 1 are allowed.
        pub redshift_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetRedshiftTarget>,
        >,
        /// Parameters used when you are providing retry policies. Documented below. A maximum of 1 are allowed.
        pub retry_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetRetryPolicy>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. Required if `ecs_target` is used or target in `arn` is EC2 instance, Kinesis data stream, Step Functions state machine, or Event Bus in different account or region.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the rule you want to add targets to.
        ///
        /// The following arguments are optional:
        pub rule: pulumi_gestalt_rust::Output<String>,
        /// Parameters used when you are using the rule to invoke Amazon EC2 Run Command. Documented below. A maximum of 5 are allowed.
        pub run_command_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cloudwatch::EventTargetRunCommandTarget>>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SageMaker Pipeline. Documented below. A maximum of 1 are allowed.
        pub sagemaker_pipeline_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetSagemakerPipelineTarget>,
        >,
        /// Parameters used when you are using the rule to invoke an Amazon SQS Queue. Documented below. A maximum of 1 are allowed.
        pub sqs_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudwatch::EventTargetSqsTarget>,
        >,
        /// The unique target assignment ID. If missing, will generate a random, unique id.
        pub target_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventTargetArgs,
    ) -> EventTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let appsync_target_binding = args.appsync_target.get_output(context);
        let arn_binding = args.arn.get_output(context);
        let batch_target_binding = args.batch_target.get_output(context);
        let dead_letter_config_binding = args.dead_letter_config.get_output(context);
        let ecs_target_binding = args.ecs_target.get_output(context);
        let event_bus_name_binding = args.event_bus_name.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let http_target_binding = args.http_target.get_output(context);
        let input_binding = args.input.get_output(context);
        let input_path_binding = args.input_path.get_output(context);
        let input_transformer_binding = args.input_transformer.get_output(context);
        let kinesis_target_binding = args.kinesis_target.get_output(context);
        let redshift_target_binding = args.redshift_target.get_output(context);
        let retry_policy_binding = args.retry_policy.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let rule_binding = args.rule.get_output(context);
        let run_command_targets_binding = args.run_command_targets.get_output(context);
        let sagemaker_pipeline_target_binding = args
            .sagemaker_pipeline_target
            .get_output(context);
        let sqs_target_binding = args.sqs_target.get_output(context);
        let target_id_binding = args.target_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventTarget:EventTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appsyncTarget".into(),
                    value: &appsync_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchTarget".into(),
                    value: &batch_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterConfig".into(),
                    value: &dead_letter_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ecsTarget".into(),
                    value: &ecs_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventBusName".into(),
                    value: &event_bus_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpTarget".into(),
                    value: &http_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "input".into(),
                    value: &input_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputPath".into(),
                    value: &input_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputTransformer".into(),
                    value: &input_transformer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kinesisTarget".into(),
                    value: &kinesis_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redshiftTarget".into(),
                    value: &redshift_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retryPolicy".into(),
                    value: &retry_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rule".into(),
                    value: &rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runCommandTargets".into(),
                    value: &run_command_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sagemakerPipelineTarget".into(),
                    value: &sagemaker_pipeline_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqsTarget".into(),
                    value: &sqs_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventTargetResult {
            appsync_target: o.get_field("appsyncTarget"),
            arn: o.get_field("arn"),
            batch_target: o.get_field("batchTarget"),
            dead_letter_config: o.get_field("deadLetterConfig"),
            ecs_target: o.get_field("ecsTarget"),
            event_bus_name: o.get_field("eventBusName"),
            force_destroy: o.get_field("forceDestroy"),
            http_target: o.get_field("httpTarget"),
            input: o.get_field("input"),
            input_path: o.get_field("inputPath"),
            input_transformer: o.get_field("inputTransformer"),
            kinesis_target: o.get_field("kinesisTarget"),
            redshift_target: o.get_field("redshiftTarget"),
            retry_policy: o.get_field("retryPolicy"),
            role_arn: o.get_field("roleArn"),
            rule: o.get_field("rule"),
            run_command_targets: o.get_field("runCommandTargets"),
            sagemaker_pipeline_target: o.get_field("sagemakerPipelineTarget"),
            sqs_target: o.get_field("sqsTarget"),
            target_id: o.get_field("targetId"),
        }
    }
}
