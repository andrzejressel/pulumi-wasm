#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the ECS Cluster
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// ARN of the ECS Cluster
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Number of pending tasks for the ECS Cluster
        pub pending_tasks_count: pulumi_gestalt_rust::Output<i32>,
        /// The number of registered container instances for the ECS Cluster
        pub registered_container_instances_count: pulumi_gestalt_rust::Output<i32>,
        /// Number of running tasks for the ECS Cluster
        pub running_tasks_count: pulumi_gestalt_rust::Output<i32>,
        /// The default Service Connect namespace
        pub service_connect_defaults: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ecs::GetClusterServiceConnectDefault>,
        >,
        /// Settings associated with the ECS Cluster
        pub settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ecs::GetClusterSetting>,
        >,
        /// Status of the ECS Cluster
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding_1 = args.cluster_name.get_output(context);
        let cluster_name_binding = cluster_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            pending_tasks_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pendingTasksCount"),
            ),
            registered_container_instances_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registeredContainerInstancesCount"),
            ),
            running_tasks_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runningTasksCount"),
            ),
            service_connect_defaults: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceConnectDefaults"),
            ),
            settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("settings"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
