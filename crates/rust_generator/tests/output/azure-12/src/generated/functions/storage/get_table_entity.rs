#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_table_entity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableEntityArgs {
        /// The key for the partition where the entity will be retrieved.
        #[builder(into)]
        pub partition_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The key for the row where the entity will be retrieved.
        #[builder(into)]
        pub row_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Storage Table ID where the entity exists.
        #[builder(into)]
        pub storage_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableEntityResult {
        /// A map of key/value pairs that describe the entity to be stored in the storage table.
        pub entity: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub partition_key: pulumi_gestalt_rust::Output<String>,
        pub row_key: pulumi_gestalt_rust::Output<String>,
        pub storage_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTableEntityArgs,
    ) -> GetTableEntityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let partition_key_binding = args.partition_key.get_output(context);
        let row_key_binding = args.row_key.get_output(context);
        let storage_table_id_binding = args.storage_table_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getTableEntity:getTableEntity".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionKey".into(),
                    value: partition_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rowKey".into(),
                    value: row_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageTableId".into(),
                    value: storage_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTableEntityResult {
            entity: o.get_field("entity"),
            id: o.get_field("id"),
            partition_key: o.get_field("partitionKey"),
            row_key: o.get_field("rowKey"),
            storage_table_id: o.get_field("storageTableId"),
        }
    }
}
