/// Resource for managing an Athena Prepared Statement.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod prepared_statement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreparedStatementArgs {
        /// Brief explanation of prepared statement. Maximum length of 1024.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the prepared statement. Maximum length of 256.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The query string for the prepared statement.
        #[builder(into)]
        pub query_statement: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the workgroup to which the prepared statement belongs.
        #[builder(into)]
        pub workgroup: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PreparedStatementResult {
        /// Brief explanation of prepared statement. Maximum length of 1024.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the prepared statement. Maximum length of 256.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The query string for the prepared statement.
        pub query_statement: pulumi_gestalt_rust::Output<String>,
        /// The name of the workgroup to which the prepared statement belongs.
        pub workgroup: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreparedStatementArgs,
    ) -> PreparedStatementResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_statement_binding = args.query_statement.get_output(context);
        let workgroup_binding = args.workgroup.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:athena/preparedStatement:PreparedStatement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryStatement".into(),
                    value: &query_statement_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workgroup".into(),
                    value: &workgroup_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PreparedStatementResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            query_statement: o.get_field("queryStatement"),
            workgroup: o.get_field("workgroup"),
        }
    }
}
