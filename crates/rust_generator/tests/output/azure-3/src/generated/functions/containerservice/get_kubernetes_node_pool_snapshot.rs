#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kubernetes_node_pool_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKubernetesNodePoolSnapshotArgs {
        /// The name of the Kubernetes Node Pool Snapshot.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Kubernetes Node Pool Snapshot exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKubernetesNodePoolSnapshotResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the source Node Pool.
        pub source_node_pool_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKubernetesNodePoolSnapshotArgs,
    ) -> GetKubernetesNodePoolSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getKubernetesNodePoolSnapshot:getKubernetesNodePoolSnapshot"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKubernetesNodePoolSnapshotResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_node_pool_id: o.get_field("sourceNodePoolId"),
            tags: o.get_field("tags"),
        }
    }
}
