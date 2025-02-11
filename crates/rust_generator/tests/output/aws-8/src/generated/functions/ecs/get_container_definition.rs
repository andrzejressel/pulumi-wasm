#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_container_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainerDefinitionArgs {
        /// Name of the container definition
        #[builder(into)]
        pub container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the task definition which contains the container
        #[builder(into)]
        pub task_definition: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContainerDefinitionResult {
        pub container_name: pulumi_gestalt_rust::Output<String>,
        /// CPU limit for this container definition
        pub cpu: pulumi_gestalt_rust::Output<i32>,
        /// Indicator if networking is disabled
        pub disable_networking: pulumi_gestalt_rust::Output<bool>,
        /// Set docker labels
        pub docker_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Environment in use
        pub environment: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Docker image in use, including the digest
        pub image: pulumi_gestalt_rust::Output<String>,
        /// Digest of the docker image in use
        pub image_digest: pulumi_gestalt_rust::Output<String>,
        /// Memory limit for this container definition
        pub memory: pulumi_gestalt_rust::Output<i32>,
        /// Soft limit (in MiB) of memory to reserve for the container. When system memory is under contention, Docker attempts to keep the container memory to this soft limit
        pub memory_reservation: pulumi_gestalt_rust::Output<i32>,
        pub task_definition: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContainerDefinitionArgs,
    ) -> GetContainerDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_name_binding = args.container_name.get_output(context);
        let task_definition_binding = args.task_definition.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecs/getContainerDefinition:getContainerDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskDefinition".into(),
                    value: &task_definition_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContainerDefinitionResult {
            container_name: o.get_field("containerName"),
            cpu: o.get_field("cpu"),
            disable_networking: o.get_field("disableNetworking"),
            docker_labels: o.get_field("dockerLabels"),
            environment: o.get_field("environment"),
            id: o.get_field("id"),
            image: o.get_field("image"),
            image_digest: o.get_field("imageDigest"),
            memory: o.get_field("memory"),
            memory_reservation: o.get_field("memoryReservation"),
            task_definition: o.get_field("taskDefinition"),
        }
    }
}
