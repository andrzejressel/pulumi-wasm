#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_db_nodes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbNodesArgs {
        /// The id of the Cloud VM cluster.
        #[builder(into)]
        pub cloud_vm_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDbNodesResult {
        pub cloud_vm_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// A `db_nodes` block as defined below.
        pub db_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::oracle::GetDbNodesDbNode>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDbNodesArgs,
    ) -> GetDbNodesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_vm_cluster_id_binding = args.cloud_vm_cluster_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:oracle/getDbNodes:getDbNodes".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudVmClusterId".into(),
                    value: &cloud_vm_cluster_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDbNodesResult {
            cloud_vm_cluster_id: o.get_field("cloudVmClusterId"),
            db_nodes: o.get_field("dbNodes"),
            id: o.get_field("id"),
        }
    }
}
