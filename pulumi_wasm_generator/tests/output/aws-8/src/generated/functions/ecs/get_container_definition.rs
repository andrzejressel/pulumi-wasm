pub mod get_container_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainerDefinitionArgs {
        /// Name of the container definition
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the task definition which contains the container
        #[builder(into)]
        pub task_definition: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContainerDefinitionResult {
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// CPU limit for this container definition
        pub cpu: pulumi_wasm_rust::Output<i32>,
        /// Indicator if networking is disabled
        pub disable_networking: pulumi_wasm_rust::Output<bool>,
        /// Set docker labels
        pub docker_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Environment in use
        pub environment: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Docker image in use, including the digest
        pub image: pulumi_wasm_rust::Output<String>,
        /// Digest of the docker image in use
        pub image_digest: pulumi_wasm_rust::Output<String>,
        /// Memory limit for this container definition
        pub memory: pulumi_wasm_rust::Output<i32>,
        /// Soft limit (in MiB) of memory to reserve for the container. When system memory is under contention, Docker attempts to keep the container memory to this soft limit
        pub memory_reservation: pulumi_wasm_rust::Output<i32>,
        pub task_definition: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetContainerDefinitionArgs,
    ) -> GetContainerDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_name_binding = args.container_name.get_output(context).get_inner();
        let task_definition_binding = args
            .task_definition
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecs/getContainerDefinition:getContainerDefinition".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "taskDefinition".into(),
                    value: &task_definition_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetContainerDefinitionResult {
            container_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerName"),
            ),
            cpu: pulumi_wasm_rust::__private::into_domain(o.extract_field("cpu")),
            disable_networking: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableNetworking"),
            ),
            docker_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dockerLabels"),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image: pulumi_wasm_rust::__private::into_domain(o.extract_field("image")),
            image_digest: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageDigest"),
            ),
            memory: pulumi_wasm_rust::__private::into_domain(o.extract_field("memory")),
            memory_reservation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("memoryReservation"),
            ),
            task_definition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("taskDefinition"),
            ),
        }
    }
}
