#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dataset_kusto_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetKustoDatabaseArgs {
        /// The name of this Data Share Kusto Database Dataset.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Data Share where this Data Share Kusto Database Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetKustoDatabaseResult {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location of the Kusto Cluster.
        pub kusto_cluster_location: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Kusto Cluster Database to be shared with the receiver.
        pub kusto_database_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub share_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatasetKustoDatabaseArgs,
    ) -> GetDatasetKustoDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let share_id_binding = args.share_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:datashare/getDatasetKustoDatabase:getDatasetKustoDatabase"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shareId".into(),
                    value: share_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatasetKustoDatabaseResult {
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            kusto_cluster_location: o.get_field("kustoClusterLocation"),
            kusto_database_id: o.get_field("kustoDatabaseId"),
            name: o.get_field("name"),
            share_id: o.get_field("shareId"),
        }
    }
}
