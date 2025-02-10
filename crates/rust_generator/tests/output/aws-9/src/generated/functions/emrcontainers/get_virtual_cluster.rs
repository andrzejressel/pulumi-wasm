#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualClusterArgs {
        /// Key-value mapping of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the cluster.
        #[builder(into)]
        pub virtual_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualClusterResult {
        /// ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing information about the underlying container provider (EKS cluster) for your EMR Containers cluster.
        pub container_providers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::emrcontainers::GetVirtualClusterContainerProvider,
            >,
        >,
        /// Unix epoch time stamp in seconds for when the cluster was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the cluster.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of the EKS cluster. One of `RUNNING`, `TERMINATING`, `TERMINATED`, `ARRESTED`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub virtual_cluster_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualClusterArgs,
    ) -> GetVirtualClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let virtual_cluster_id_binding = args.virtual_cluster_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:emrcontainers/getVirtualCluster:getVirtualCluster".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualClusterId".into(),
                    value: virtual_cluster_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualClusterResult {
            arn: o.get_field("arn"),
            container_providers: o.get_field("containerProviders"),
            created_at: o.get_field("createdAt"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            virtual_cluster_id: o.get_field("virtualClusterId"),
        }
    }
}
