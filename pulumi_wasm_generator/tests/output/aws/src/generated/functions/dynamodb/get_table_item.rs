pub mod get_table_item {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableItemArgs {
        #[builder(into, default)]
        pub expression_attribute_names: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of attribute names to AttributeValue objects, representing the primary key of the item to retrieve.
        /// For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.
        /// If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.
        #[builder(into, default)]
        pub projection_expression: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the table containing the requested item.
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableItemResult {
        pub expression_attribute_names: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// JSON representation of a map of attribute names to [AttributeValue](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html) objects, as specified by ProjectionExpression.
        pub item: pulumi_wasm_rust::Output<String>,
        pub key: pulumi_wasm_rust::Output<String>,
        pub projection_expression: pulumi_wasm_rust::Output<Option<String>>,
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTableItemArgs) -> GetTableItemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let expression_attribute_names_binding = args
            .expression_attribute_names
            .get_inner();
        let key_binding = args.key.get_inner();
        let projection_expression_binding = args.projection_expression.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dynamodb/getTableItem:getTableItem".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expressionAttributeNames".into(),
                    value: &expression_attribute_names_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "projectionExpression".into(),
                    value: &projection_expression_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "expressionAttributeNames".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "item".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "projectionExpression".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTableItemResult {
            expression_attribute_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressionAttributeNames").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            item: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("item").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            projection_expression: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectionExpression").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}