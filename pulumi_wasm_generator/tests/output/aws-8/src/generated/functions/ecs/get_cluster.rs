pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the ECS Cluster
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// ARN of the ECS Cluster
        pub arn: pulumi_wasm_rust::Output<String>,
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Number of pending tasks for the ECS Cluster
        pub pending_tasks_count: pulumi_wasm_rust::Output<i32>,
        /// The number of registered container instances for the ECS Cluster
        pub registered_container_instances_count: pulumi_wasm_rust::Output<i32>,
        /// Number of running tasks for the ECS Cluster
        pub running_tasks_count: pulumi_wasm_rust::Output<i32>,
        /// The default Service Connect namespace
        pub service_connect_defaults: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ecs::GetClusterServiceConnectDefault>,
        >,
        /// Settings associated with the ECS Cluster
        pub settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ecs::GetClusterSetting>,
        >,
        /// Status of the ECS Cluster
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecs/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
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
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "pendingTasksCount".into(),
                },
                register_interface::ResultField {
                    name: "registeredContainerInstancesCount".into(),
                },
                register_interface::ResultField {
                    name: "runningTasksCount".into(),
                },
                register_interface::ResultField {
                    name: "serviceConnectDefaults".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            pending_tasks_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pendingTasksCount").unwrap(),
            ),
            registered_container_instances_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registeredContainerInstancesCount").unwrap(),
            ),
            running_tasks_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runningTasksCount").unwrap(),
            ),
            service_connect_defaults: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceConnectDefaults").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
