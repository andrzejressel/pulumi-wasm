/// Provides a WAF Regional Regex Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = regex_match_set::create(
///         "example",
///         RegexMatchSetArgs::builder()
///             .name("example")
///             .regex_match_tuples(
///                 vec![
///                     RegexMatchSetRegexMatchTuple::builder()
///                     .fieldToMatch(RegexMatchSetRegexMatchTupleFieldToMatch::builder()
///                     .data("User-Agent"). type ("HEADER").build_struct())
///                     .regexPatternSetId("${exampleRegexPatternSet.id}")
///                     .textTransformation("NONE").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleRegexPatternSet = regex_pattern_set::create(
///         "exampleRegexPatternSet",
///         RegexPatternSetArgs::builder()
///             .name("example")
///             .regex_pattern_strings(vec!["one", "two",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Regex Match Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/regexMatchSet:RegexMatchSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod regex_match_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexMatchSetArgs {
        /// The name or description of the Regex Match Set.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The regular expression pattern that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings. See below.
        #[builder(into, default)]
        pub regex_match_tuples: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::RegexMatchSetRegexMatchTuple>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegexMatchSetResult {
        /// The name or description of the Regex Match Set.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The regular expression pattern that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings. See below.
        pub regex_match_tuples: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::RegexMatchSetRegexMatchTuple>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegexMatchSetArgs) -> RegexMatchSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let regex_match_tuples_binding = args.regex_match_tuples.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/regexMatchSet:RegexMatchSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "regexMatchTuples".into(),
                    value: &regex_match_tuples_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "regexMatchTuples".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegexMatchSetResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            regex_match_tuples: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regexMatchTuples").unwrap(),
            ),
        }
    }
}