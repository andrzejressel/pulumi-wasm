/// Provides an Athena Named Query resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = named_query::create(
///         "foo",
///         NamedQueryArgs::builder()
///             .database("${hogeDatabase.name}")
///             .name("bar")
///             .query("SELECT * FROM ${hogeDatabase.name} limit 10;")
///             .workgroup("${testWorkgroup.id}")
///             .build_struct(),
///     );
///     let hoge = bucket_v_2::create(
///         "hoge",
///         BucketV2Args::builder().bucket("tf-test").build_struct(),
///     );
///     let hogeDatabase = database::create(
///         "hogeDatabase",
///         DatabaseArgs::builder().bucket("${hoge.id}").name("users").build_struct(),
///     );
///     let test = key::create(
///         "test",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Athena KMS Key")
///             .build_struct(),
///     );
///     let testWorkgroup = workgroup::create(
///         "testWorkgroup",
///         WorkgroupArgs::builder()
///             .configuration(
///                 WorkgroupConfiguration::builder()
///                     .resultConfiguration(
///                         WorkgroupConfigurationResultConfiguration::builder()
///                             .encryptionConfiguration(
///                                 WorkgroupConfigurationResultConfigurationEncryptionConfiguration::builder()
///                                     .encryptionOption("SSE_KMS")
///                                     .kmsKeyArn("${test.arn}")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Athena Named Query using the query ID. For example:
///
/// ```sh
/// $ pulumi import aws:athena/namedQuery:NamedQuery example 0123456789
/// ```
pub mod named_query {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamedQueryArgs {
        /// Database to which the query belongs.
        #[builder(into)]
        pub database: pulumi_wasm_rust::InputOrOutput<String>,
        /// Brief explanation of the query. Maximum length of 1024.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Plain language name for the query. Maximum length of 128.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Text of the query itself. In other words, all query statements. Maximum length of 262144.
        #[builder(into)]
        pub query: pulumi_wasm_rust::InputOrOutput<String>,
        /// Workgroup to which the query belongs. Defaults to `primary`
        #[builder(into, default)]
        pub workgroup: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NamedQueryResult {
        /// Database to which the query belongs.
        pub database: pulumi_wasm_rust::Output<String>,
        /// Brief explanation of the query. Maximum length of 1024.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Plain language name for the query. Maximum length of 128.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Text of the query itself. In other words, all query statements. Maximum length of 262144.
        pub query: pulumi_wasm_rust::Output<String>,
        /// Workgroup to which the query belongs. Defaults to `primary`
        pub workgroup: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NamedQueryArgs,
    ) -> NamedQueryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_binding = args.database.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let query_binding = args.query.get_output(context).get_inner();
        let workgroup_binding = args.workgroup.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:athena/namedQuery:NamedQuery".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "query".into(),
                    value: &query_binding,
                },
                register_interface::ObjectField {
                    name: "workgroup".into(),
                    value: &workgroup_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamedQueryResult {
            database: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            query: pulumi_wasm_rust::__private::into_domain(o.extract_field("query")),
            workgroup: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workgroup"),
            ),
        }
    }
}
