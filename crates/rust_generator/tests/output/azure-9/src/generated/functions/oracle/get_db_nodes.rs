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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDbNodesArgs,
    ) -> GetDbNodesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cloud_vm_cluster_id_binding_1 = args.cloud_vm_cluster_id.get_output(context);
        let cloud_vm_cluster_id_binding = cloud_vm_cluster_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getDbNodes:getDbNodes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudVmClusterId".into(),
                    value: &cloud_vm_cluster_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDbNodesResult {
            cloud_vm_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudVmClusterId"),
            ),
            db_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbNodes"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
