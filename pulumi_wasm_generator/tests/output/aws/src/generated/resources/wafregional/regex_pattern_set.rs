/// Provides a WAF Regional Regex Pattern Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = regex_pattern_set::create(
///         "example",
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
/// Using `pulumi import`, import WAF Regional Regex Pattern Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/regexPatternSet:RegexPatternSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod regex_pattern_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexPatternSetArgs {
        /// The name or description of the Regex Pattern Set.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of regular expression (regex) patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`.
        #[builder(into, default)]
        pub regex_pattern_strings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct RegexPatternSetResult {
        /// The name or description of the Regex Pattern Set.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of regular expression (regex) patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`.
        pub regex_pattern_strings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegexPatternSetArgs) -> RegexPatternSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let regex_pattern_strings_binding = args.regex_pattern_strings.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/regexPatternSet:RegexPatternSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "regexPatternStrings".into(),
                    value: &regex_pattern_strings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "regexPatternStrings".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegexPatternSetResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            regex_pattern_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regexPatternStrings").unwrap(),
            ),
        }
    }
}
