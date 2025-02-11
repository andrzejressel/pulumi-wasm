/// Provides a DynamoDB table item resource
///
/// > **Note:** This resource is not meant to be used for managing large amounts of data in your table, it is not designed to scale.
///   You should perform **regular backups** of all data in the table, see [AWS docs for more](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/BackupRestore.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table_item::create(
///         "example",
///         TableItemArgs::builder()
///             .hash_key("${exampleTable.hashKey}")
///             .item(
///                 "{\n  \"exampleHashKey\": {\"S\": \"something\"},\n  \"one\": {\"N\": \"11111\"},\n  \"two\": {\"N\": \"22222\"},\n  \"three\": {\"N\": \"33333\"},\n  \"four\": {\"N\": \"44444\"}\n}\n",
///             )
///             .table_name("${exampleTable.name}")
///             .build_struct(),
///     );
///     let exampleTable = table::create(
///         "exampleTable",
///         TableArgs::builder()
///             .attributes(
///                 vec![
///                     TableAttribute::builder().name("exampleHashKey"). type ("S")
///                     .build_struct(),
///                 ],
///             )
///             .hash_key("exampleHashKey")
///             .name("example-name")
///             .read_capacity(10)
///             .write_capacity(10)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import DynamoDB table items.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table_item {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableItemArgs {
        /// Hash key to use for lookups and identification of the item
        #[builder(into)]
        pub hash_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// JSON representation of a map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.
        #[builder(into)]
        pub item: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Range key to use for lookups and identification of the item. Required if there is range key defined in the table.
        #[builder(into, default)]
        pub range_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the table to contain the item.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TableItemResult {
        /// Hash key to use for lookups and identification of the item
        pub hash_key: pulumi_gestalt_rust::Output<String>,
        /// JSON representation of a map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.
        pub item: pulumi_gestalt_rust::Output<String>,
        /// Range key to use for lookups and identification of the item. Required if there is range key defined in the table.
        pub range_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the table to contain the item.
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableItemArgs,
    ) -> TableItemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hash_key_binding = args.hash_key.get_output(context);
        let item_binding = args.item.get_output(context);
        let range_key_binding = args.range_key.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dynamodb/tableItem:TableItem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hashKey".into(),
                    value: &hash_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "item".into(),
                    value: &item_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rangeKey".into(),
                    value: &range_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableItemResult {
            hash_key: o.get_field("hashKey"),
            item: o.get_field("item"),
            range_key: o.get_field("rangeKey"),
            table_name: o.get_field("tableName"),
        }
    }
}
