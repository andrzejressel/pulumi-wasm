/// Resource for managing an Athena Prepared Statement.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = bucket_v_2::create(
///         "test",
///         BucketV2Args::builder().bucket("tf-test").force_destroy(true).build_struct(),
///     );
///     let testDatabase = database::create(
///         "testDatabase",
///         DatabaseArgs::builder().bucket("${test.bucket}").name("example").build_struct(),
///     );
///     let testPreparedStatement = prepared_statement::create(
///         "testPreparedStatement",
///         PreparedStatementArgs::builder()
///             .name("tf_test")
///             .query_statement("SELECT * FROM ${testDatabase.name} WHERE x = ?")
///             .workgroup("${testWorkgroup.name}")
///             .build_struct(),
///     );
///     let testWorkgroup = workgroup::create(
///         "testWorkgroup",
///         WorkgroupArgs::builder().name("tf-test").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Athena Prepared Statement using the `WORKGROUP-NAME/STATEMENT-NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:athena/preparedStatement:PreparedStatement example 12345abcde/example
/// ```
pub mod prepared_statement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreparedStatementArgs {
        /// Brief explanation of prepared statement. Maximum length of 1024.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the prepared statement. Maximum length of 256.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The query string for the prepared statement.
        #[builder(into)]
        pub query_statement: pulumi_wasm_rust::Output<String>,
        /// The name of the workgroup to which the prepared statement belongs.
        #[builder(into)]
        pub workgroup: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PreparedStatementResult {
        /// Brief explanation of prepared statement. Maximum length of 1024.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the prepared statement. Maximum length of 256.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The query string for the prepared statement.
        pub query_statement: pulumi_wasm_rust::Output<String>,
        /// The name of the workgroup to which the prepared statement belongs.
        pub workgroup: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PreparedStatementArgs) -> PreparedStatementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let query_statement_binding = args.query_statement.get_inner();
        let workgroup_binding = args.workgroup.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:athena/preparedStatement:PreparedStatement".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryStatement".into(),
                    value: &query_statement_binding,
                },
                register_interface::ObjectField {
                    name: "workgroup".into(),
                    value: &workgroup_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queryStatement".into(),
                },
                register_interface::ResultField {
                    name: "workgroup".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PreparedStatementResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query_statement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryStatement").unwrap(),
            ),
            workgroup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workgroup").unwrap(),
            ),
        }
    }
}
