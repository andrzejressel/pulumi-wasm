#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_task_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTaskDefinitionArgs {
        /// Family for the latest ACTIVE revision, family and revision (family:revision) for a specific revision in the family, the ARN of the task definition to access to.
        #[builder(into)]
        pub task_definition: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTaskDefinitionResult {
        /// ARN of the task definition.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Task Definition with the trailing `revision` removed. This may be useful for situations where the latest task definition is always desired. If a revision isn't specified, the latest ACTIVE revision is used. See the [AWS documentation](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_StartTask.html#ECS-StartTask-request-taskDefinition) for details.
        pub arn_without_revision: pulumi_gestalt_rust::Output<String>,
        /// ARN of the task execution role that the Amazon ECS container agent and the Docker.
        pub execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Family of this task definition.
        pub family: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Docker networking mode to use for the containers in this task.
        pub network_mode: pulumi_gestalt_rust::Output<String>,
        /// Revision of this task definition.
        pub revision: pulumi_gestalt_rust::Output<i32>,
        /// Status of this task definition.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub task_definition: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role that containers in this task can assume.
        pub task_role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTaskDefinitionArgs,
    ) -> GetTaskDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let task_definition_binding = args.task_definition.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecs/getTaskDefinition:getTaskDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskDefinition".into(),
                    value: &task_definition_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTaskDefinitionResult {
            arn: o.get_field("arn"),
            arn_without_revision: o.get_field("arnWithoutRevision"),
            execution_role_arn: o.get_field("executionRoleArn"),
            family: o.get_field("family"),
            id: o.get_field("id"),
            network_mode: o.get_field("networkMode"),
            revision: o.get_field("revision"),
            status: o.get_field("status"),
            task_definition: o.get_field("taskDefinition"),
            task_role_arn: o.get_field("taskRoleArn"),
        }
    }
}
