pub mod get_container_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainerDefinitionArgs {
        /// Name of the container definition
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the task definition which contains the container
        #[builder(into)]
        pub task_definition: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetContainerDefinitionArgs) -> GetContainerDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_name_binding = args.container_name.get_inner();
        let task_definition_binding = args.task_definition.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecs/getContainerDefinition:getContainerDefinition".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerName".into(),
                },
                register_interface::ResultField {
                    name: "cpu".into(),
                },
                register_interface::ResultField {
                    name: "disableNetworking".into(),
                },
                register_interface::ResultField {
                    name: "dockerLabels".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "image".into(),
                },
                register_interface::ResultField {
                    name: "imageDigest".into(),
                },
                register_interface::ResultField {
                    name: "memory".into(),
                },
                register_interface::ResultField {
                    name: "memoryReservation".into(),
                },
                register_interface::ResultField {
                    name: "taskDefinition".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetContainerDefinitionResult {
            container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerName").unwrap(),
            ),
            cpu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpu").unwrap(),
            ),
            disable_networking: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableNetworking").unwrap(),
            ),
            docker_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerLabels").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("image").unwrap(),
            ),
            image_digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageDigest").unwrap(),
            ),
            memory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memory").unwrap(),
            ),
            memory_reservation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memoryReservation").unwrap(),
            ),
            task_definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskDefinition").unwrap(),
            ),
        }
    }
}
