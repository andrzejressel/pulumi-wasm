pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// ARN of the ECS Cluster
        #[builder(into)]
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the ECS Service
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// ARN of the ECS Service
        pub arn: pulumi_wasm_rust::Output<String>,
        pub availability_zone_rebalancing: pulumi_wasm_rust::Output<String>,
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// Number of tasks for the ECS Service
        pub desired_count: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Launch type for the ECS Service
        pub launch_type: pulumi_wasm_rust::Output<String>,
        /// Scheduling strategy for the ECS Service
        pub scheduling_strategy: pulumi_wasm_rust::Output<String>,
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Family for the latest ACTIVE revision or full ARN of the task definition.
        pub task_definition: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_inner();
        let service_name_binding = args.service_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecs/getService:getService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneRebalancing".into(),
                },
                register_interface::ResultField {
                    name: "clusterArn".into(),
                },
                register_interface::ResultField {
                    name: "desiredCount".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "launchType".into(),
                },
                register_interface::ResultField {
                    name: "schedulingStrategy".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
        GetServiceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone_rebalancing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneRebalancing").unwrap(),
            ),
            cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterArn").unwrap(),
            ),
            desired_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredCount").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            launch_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchType").unwrap(),
            ),
            scheduling_strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulingStrategy").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            task_definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskDefinition").unwrap(),
            ),
        }
    }
}
