#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_table_entities {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableEntitiesArgs {
        /// The filter used to retrieve the entities.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of properties to select from the returned Storage Table Entities.
        #[builder(into, default)]
        pub selects: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Storage Table ID where the entities exist.
        #[builder(into)]
        pub storage_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableEntitiesResult {
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of `items` blocks as defined below.
        pub items: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetTableEntitiesItem>,
        >,
        pub selects: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub storage_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTableEntitiesArgs,
    ) -> GetTableEntitiesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let selects_binding = args.selects.get_output(context);
        let storage_table_id_binding = args.storage_table_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getTableEntities:getTableEntities".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selects".into(),
                    value: selects_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageTableId".into(),
                    value: storage_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTableEntitiesResult {
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            items: o.get_field("items"),
            selects: o.get_field("selects"),
            storage_table_id: o.get_field("storageTableId"),
        }
    }
}
