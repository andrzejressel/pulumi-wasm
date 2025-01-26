/// Provides a WAF Regional SQL Injection Match Set Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// Using `pulumi import`, import WAF Regional Sql Injection Match Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/sqlInjectionMatchSet:SqlInjectionMatchSet sql_injection_match_set a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod sql_injection_match_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlInjectionMatchSetArgs {
        /// The name or description of the SizeConstraintSet.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The parts of web requests that you want AWS WAF to inspect for malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.
        #[builder(into, default)]
        pub sql_injection_match_tuples: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::wafregional::SqlInjectionMatchSetSqlInjectionMatchTuple,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct SqlInjectionMatchSetResult {
        /// The name or description of the SizeConstraintSet.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parts of web requests that you want AWS WAF to inspect for malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.
        pub sql_injection_match_tuples: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::wafregional::SqlInjectionMatchSetSqlInjectionMatchTuple,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SqlInjectionMatchSetArgs,
    ) -> SqlInjectionMatchSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let sql_injection_match_tuples_binding = args
            .sql_injection_match_tuples
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/sqlInjectionMatchSet:SqlInjectionMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sqlInjectionMatchTuples".into(),
                    value: &sql_injection_match_tuples_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlInjectionMatchSetResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            sql_injection_match_tuples: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sqlInjectionMatchTuples"),
            ),
        }
    }
}
