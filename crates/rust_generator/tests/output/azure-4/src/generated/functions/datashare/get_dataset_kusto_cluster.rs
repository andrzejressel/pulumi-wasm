#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dataset_kusto_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetKustoClusterArgs {
        /// The name of this Data Share Kusto Cluster Dataset.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Data Share where this Data Share Kusto Cluster Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetKustoClusterResult {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Kusto Cluster to be shared with the receiver.
        pub kusto_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The location of the Kusto Cluster.
        pub kusto_cluster_location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub share_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDatasetKustoClusterArgs,
    ) -> GetDatasetKustoClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let share_id_binding = args.share_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getDatasetKustoCluster:getDatasetKustoCluster"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shareId".into(),
                    value: &share_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatasetKustoClusterResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kusto_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kustoClusterId"),
            ),
            kusto_cluster_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kustoClusterLocation"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            share_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareId"),
            ),
        }
    }
}
