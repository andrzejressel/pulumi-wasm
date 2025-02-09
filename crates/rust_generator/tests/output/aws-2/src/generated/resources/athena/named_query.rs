/// Provides an Athena Named Query resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod named_query {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamedQueryArgs {
        /// Database to which the query belongs.
        #[builder(into)]
        pub database: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Brief explanation of the query. Maximum length of 1024.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Plain language name for the query. Maximum length of 128.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Text of the query itself. In other words, all query statements. Maximum length of 262144.
        #[builder(into)]
        pub query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Workgroup to which the query belongs. Defaults to `primary`
        #[builder(into, default)]
        pub workgroup: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NamedQueryResult {
        /// Database to which the query belongs.
        pub database: pulumi_gestalt_rust::Output<String>,
        /// Brief explanation of the query. Maximum length of 1024.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Plain language name for the query. Maximum length of 128.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Text of the query itself. In other words, all query statements. Maximum length of 262144.
        pub query: pulumi_gestalt_rust::Output<String>,
        /// Workgroup to which the query belongs. Defaults to `primary`
        pub workgroup: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NamedQueryArgs,
    ) -> NamedQueryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_binding = args.database.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_binding = args.query.get_output(context);
        let workgroup_binding = args.workgroup.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:athena/namedQuery:NamedQuery".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "database".into(),
                    value: database_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "query".into(),
                    value: query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workgroup".into(),
                    value: workgroup_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NamedQueryResult {
            database: o.get_field("database"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            query: o.get_field("query"),
            workgroup: o.get_field("workgroup"),
        }
    }
}
