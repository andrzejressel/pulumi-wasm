/// Manages an Entity within a Table in an Azure Storage Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: azureexample
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: azureexamplestorage1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleTable:
///     type: azure:storage:Table
///     name: example
///     properties:
///       name: myexampletable
///       storageAccountName: ${exampleAccount.name}
///   exampleTableEntity:
///     type: azure:storage:TableEntity
///     name: example
///     properties:
///       storageTableId: ${exampleTable.id}
///       partitionKey: examplepartition
///       rowKey: examplerow
///       entity:
///         example: example
/// ```
///
/// ## Import
///
/// Entities within a Table in an Azure Storage Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/tableEntity:TableEntity entity1 https://example.table.core.windows.net/table1(PartitionKey='samplepartition',RowKey='samplerow')
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table_entity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableEntityArgs {
        /// A map of key/value pairs that describe the entity to be inserted/merged in to the storage table.
        #[builder(into)]
        pub entity: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The key for the partition where the entity will be inserted/merged. Changing this forces a new resource to be created.
        #[builder(into)]
        pub partition_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The key for the row where the entity will be inserted/merged. Changing this forces a new resource to be created.
        #[builder(into)]
        pub row_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Storage Share ID in which this file will be placed into. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TableEntityResult {
        /// A map of key/value pairs that describe the entity to be inserted/merged in to the storage table.
        pub entity: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The key for the partition where the entity will be inserted/merged. Changing this forces a new resource to be created.
        pub partition_key: pulumi_gestalt_rust::Output<String>,
        /// The key for the row where the entity will be inserted/merged. Changing this forces a new resource to be created.
        pub row_key: pulumi_gestalt_rust::Output<String>,
        /// The Storage Share ID in which this file will be placed into. Changing this forces a new resource to be created.
        pub storage_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableEntityArgs,
    ) -> TableEntityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let entity_binding = args.entity.get_output(context);
        let partition_key_binding = args.partition_key.get_output(context);
        let row_key_binding = args.row_key.get_output(context);
        let storage_table_id_binding = args.storage_table_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/tableEntity:TableEntity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entity".into(),
                    value: entity_binding.get_id(),
                },
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
        let o = context.register_resource(request);
        TableEntityResult {
            entity: o.get_field("entity"),
            partition_key: o.get_field("partitionKey"),
            row_key: o.get_field("rowKey"),
            storage_table_id: o.get_field("storageTableId"),
        }
    }
}
