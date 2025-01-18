/// Provides a DynamoDB table item resource
///
/// > **Note:** This resource is not meant to be used for managing large amounts of data in your table, it is not designed to scale.
///   You should perform **regular backups** of all data in the table, see [AWS docs for more](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/BackupRestore.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod table_item {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableItemArgs {
        /// Hash key to use for lookups and identification of the item
        #[builder(into)]
        pub hash_key: pulumi_wasm_rust::Output<String>,
        /// JSON representation of a map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.
        #[builder(into)]
        pub item: pulumi_wasm_rust::Output<String>,
        /// Range key to use for lookups and identification of the item. Required if there is range key defined in the table.
        #[builder(into, default)]
        pub range_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the table to contain the item.
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TableItemResult {
        /// Hash key to use for lookups and identification of the item
        pub hash_key: pulumi_wasm_rust::Output<String>,
        /// JSON representation of a map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.
        pub item: pulumi_wasm_rust::Output<String>,
        /// Range key to use for lookups and identification of the item. Required if there is range key defined in the table.
        pub range_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the table to contain the item.
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableItemArgs) -> TableItemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hash_key_binding = args.hash_key.get_inner();
        let item_binding = args.item.get_inner();
        let range_key_binding = args.range_key.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/tableItem:TableItem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hashKey".into(),
                    value: &hash_key_binding,
                },
                register_interface::ObjectField {
                    name: "item".into(),
                    value: &item_binding,
                },
                register_interface::ObjectField {
                    name: "rangeKey".into(),
                    value: &range_key_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "hashKey".into(),
                },
                register_interface::ResultField {
                    name: "item".into(),
                },
                register_interface::ResultField {
                    name: "rangeKey".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableItemResult {
            hash_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hashKey").unwrap(),
            ),
            item: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("item").unwrap(),
            ),
            range_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rangeKey").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}
