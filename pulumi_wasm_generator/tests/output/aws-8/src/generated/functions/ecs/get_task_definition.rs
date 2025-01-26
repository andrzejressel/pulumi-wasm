pub mod get_task_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTaskDefinitionArgs {
        /// Family for the latest ACTIVE revision, family and revision (family:revision) for a specific revision in the family, the ARN of the task definition to access to.
        #[builder(into)]
        pub task_definition: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTaskDefinitionResult {
        /// ARN of the task definition.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the Task Definition with the trailing `revision` removed. This may be useful for situations where the latest task definition is always desired. If a revision isn't specified, the latest ACTIVE revision is used. See the [AWS documentation](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_StartTask.html#ECS-StartTask-request-taskDefinition) for details.
        pub arn_without_revision: pulumi_wasm_rust::Output<String>,
        /// ARN of the task execution role that the Amazon ECS container agent and the Docker.
        pub execution_role_arn: pulumi_wasm_rust::Output<String>,
        /// Family of this task definition.
        pub family: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Docker networking mode to use for the containers in this task.
        pub network_mode: pulumi_wasm_rust::Output<String>,
        /// Revision of this task definition.
        pub revision: pulumi_wasm_rust::Output<i32>,
        /// Status of this task definition.
        pub status: pulumi_wasm_rust::Output<String>,
        pub task_definition: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that containers in this task can assume.
        pub task_role_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTaskDefinitionArgs,
    ) -> GetTaskDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let task_definition_binding = args
            .task_definition
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecs/getTaskDefinition:getTaskDefinition".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "taskDefinition".into(),
                    value: &task_definition_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTaskDefinitionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            arn_without_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("arnWithoutRevision"),
            ),
            execution_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("executionRoleArn"),
            ),
            family: pulumi_wasm_rust::__private::into_domain(o.extract_field("family")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            network_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkMode"),
            ),
            revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("revision"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            task_definition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskDefinition"),
            ),
            task_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskRoleArn"),
            ),
        }
    }
}
