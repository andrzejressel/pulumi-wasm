/// Provides a WAF Regional XSS Match Set Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let xssMatchSet = xss_match_set::create(
///         "xssMatchSet",
///         XssMatchSetArgs::builder()
///             .name("xss_match_set")
///             .xss_match_tuples(
///                 vec![
///                     XssMatchSetXssMatchTuple::builder()
///                     .fieldToMatch(XssMatchSetXssMatchTupleFieldToMatch::builder(). type
///                     ("URI").build_struct()).textTransformation("NONE").build_struct(),
///                     XssMatchSetXssMatchTuple::builder()
///                     .fieldToMatch(XssMatchSetXssMatchTupleFieldToMatch::builder(). type
///                     ("QUERY_STRING").build_struct()).textTransformation("NONE")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS WAF Regional XSS Match using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/xssMatchSet:XssMatchSet example 12345abcde
/// ```
pub mod xss_match_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct XssMatchSetArgs {
        /// The name of the set
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The parts of web requests that you want to inspect for cross-site scripting attacks.
        #[builder(into, default)]
        pub xss_match_tuples: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::XssMatchSetXssMatchTuple>>,
        >,
    }
    #[allow(dead_code)]
    pub struct XssMatchSetResult {
        /// The name of the set
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parts of web requests that you want to inspect for cross-site scripting attacks.
        pub xss_match_tuples: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::XssMatchSetXssMatchTuple>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: XssMatchSetArgs) -> XssMatchSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let xss_match_tuples_binding = args.xss_match_tuples.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/xssMatchSet:XssMatchSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "xssMatchTuples".into(),
                    value: &xss_match_tuples_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "xssMatchTuples".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        XssMatchSetResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            xss_match_tuples: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xssMatchTuples").unwrap(),
            ),
        }
    }
}
