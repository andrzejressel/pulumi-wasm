/// Provides a CloudWatch Logs query definition resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = query_definition::create(
///         "example",
///         QueryDefinitionArgs::builder()
///             .log_group_names(vec!["/aws/logGroup1", "/aws/logGroup2",])
///             .name("custom_query")
///             .query_string(
///                 "fields @timestamp, @message\n| sort @timestamp desc\n| limit 25",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch query definitions using the query definition ARN. The ARN can be found on the "Edit Query" page for the query in the AWS Console. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/queryDefinition:QueryDefinition example arn:aws:logs:us-west-2:123456789012:query-definition:269951d7-6f75-496d-9d7b-6b7a5486bdbd
/// ```
pub mod query_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueryDefinitionArgs {
        /// Specific log groups to use with the query.
        #[builder(into, default)]
        pub log_group_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the query.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The query to save. You can read more about CloudWatch Logs Query Syntax in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html).
        #[builder(into)]
        pub query_string: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct QueryDefinitionResult {
        /// Specific log groups to use with the query.
        pub log_group_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the query.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The query definition ID.
        pub query_definition_id: pulumi_wasm_rust::Output<String>,
        /// The query to save. You can read more about CloudWatch Logs Query Syntax in the [documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html).
        pub query_string: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueryDefinitionArgs) -> QueryDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_group_names_binding = args.log_group_names.get_inner();
        let name_binding = args.name.get_inner();
        let query_string_binding = args.query_string.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/queryDefinition:QueryDefinition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logGroupNames".into(),
                    value: &log_group_names_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryString".into(),
                    value: &query_string_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "logGroupNames".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queryDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "queryString".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueryDefinitionResult {
            log_group_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupNames").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryDefinitionId").unwrap(),
            ),
            query_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryString").unwrap(),
            ),
        }
    }
}