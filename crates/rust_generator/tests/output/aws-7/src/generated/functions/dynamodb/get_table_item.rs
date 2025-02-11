#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_table_item {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableItemArgs {
        #[builder(into, default)]
        pub expression_attribute_names: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of attribute names to AttributeValue objects, representing the primary key of the item to retrieve.
        /// For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.
        /// If no attribute names are specified, then all attributes are returned. If any of the requested attributes are not found, they do not appear in the result.
        #[builder(into, default)]
        pub projection_expression: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the table containing the requested item.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableItemResult {
        pub expression_attribute_names: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// JSON representation of a map of attribute names to [AttributeValue](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html) objects, as specified by ProjectionExpression.
        pub item: pulumi_gestalt_rust::Output<String>,
        pub key: pulumi_gestalt_rust::Output<String>,
        pub projection_expression: pulumi_gestalt_rust::Output<Option<String>>,
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTableItemArgs,
    ) -> GetTableItemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let expression_attribute_names_binding = args
            .expression_attribute_names
            .get_output(context);
        let key_binding = args.key.get_output(context);
        let projection_expression_binding = args
            .projection_expression
            .get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dynamodb/getTableItem:getTableItem".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressionAttributeNames".into(),
                    value: &expression_attribute_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectionExpression".into(),
                    value: &projection_expression_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTableItemResult {
            expression_attribute_names: o.get_field("expressionAttributeNames"),
            id: o.get_field("id"),
            item: o.get_field("item"),
            key: o.get_field("key"),
            projection_expression: o.get_field("projectionExpression"),
            table_name: o.get_field("tableName"),
        }
    }
}
