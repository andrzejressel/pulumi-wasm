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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TableEntityArgs,
    ) -> TableEntityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let entity_binding = args.entity.get_output(context).get_inner();
        let partition_key_binding = args.partition_key.get_output(context).get_inner();
        let row_key_binding = args.row_key.get_output(context).get_inner();
        let storage_table_id_binding = args
            .storage_table_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/tableEntity:TableEntity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "entity".into(),
                    value: &entity_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKey".into(),
                    value: &partition_key_binding,
                },
                register_interface::ObjectField {
                    name: "rowKey".into(),
                    value: &row_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageTableId".into(),
                    value: &storage_table_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TableEntityResult {
            entity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entity"),
            ),
            partition_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKey"),
            ),
            row_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rowKey"),
            ),
            storage_table_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageTableId"),
            ),
        }
    }
}
