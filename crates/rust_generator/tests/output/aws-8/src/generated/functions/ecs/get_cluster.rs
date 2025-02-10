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
        context: &pulumi_gestalt_rust::Context,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_name_binding = args.cluster_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecs/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: cluster_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetClusterResult {
            arn: o.get_field("arn"),
            cluster_name: o.get_field("clusterName"),
            id: o.get_field("id"),
            pending_tasks_count: o.get_field("pendingTasksCount"),
            registered_container_instances_count: o
                .get_field("registeredContainerInstancesCount"),
            running_tasks_count: o.get_field("runningTasksCount"),
            service_connect_defaults: o.get_field("serviceConnectDefaults"),
            settings: o.get_field("settings"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
