/// Provides a WAF SQL Injection Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sqlInjectionMatchSet = sql_injection_match_set::create(
///         "sqlInjectionMatchSet",
///         SqlInjectionMatchSetArgs::builder()
///             .name("tf-sql_injection_match_set")
///             .sql_injection_match_tuples(
///                 vec![
///                     SqlInjectionMatchSetSqlInjectionMatchTuple::builder()
///                     .fieldToMatch(SqlInjectionMatchSetSqlInjectionMatchTupleFieldToMatch::builder()
///                     . type ("QUERY_STRING").build_struct())
///                     .textTransformation("URL_DECODE").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS WAF SQL Injection Match Set using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:waf/sqlInjectionMatchSet:SqlInjectionMatchSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_injection_match_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlInjectionMatchSetArgs {
        /// The name or description of the SQL Injection Match Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parts of web requests that you want AWS WAF to inspect for malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.
        #[builder(into, default)]
        pub sql_injection_match_tuples: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::waf::SqlInjectionMatchSetSqlInjectionMatchTuple>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct SqlInjectionMatchSetResult {
        /// The name or description of the SQL Injection Match Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parts of web requests that you want AWS WAF to inspect for malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.
        pub sql_injection_match_tuples: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::waf::SqlInjectionMatchSetSqlInjectionMatchTuple>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlInjectionMatchSetArgs,
    ) -> SqlInjectionMatchSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let sql_injection_match_tuples_binding = args
            .sql_injection_match_tuples
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:waf/sqlInjectionMatchSet:SqlInjectionMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlInjectionMatchTuples".into(),
                    value: sql_injection_match_tuples_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlInjectionMatchSetResult {
            name: o.get_field("name"),
            sql_injection_match_tuples: o.get_field("sqlInjectionMatchTuples"),
        }
    }
}
