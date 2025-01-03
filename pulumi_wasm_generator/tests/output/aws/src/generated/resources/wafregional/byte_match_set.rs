/// Provides a WAF Regional Byte Match Set Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let byteSet = byte_match_set::create(
///         "byteSet",
///         ByteMatchSetArgs::builder()
///             .byte_match_tuples(
///                 vec![
///                     ByteMatchSetByteMatchTuple::builder()
///                     .fieldToMatch(ByteMatchSetByteMatchTupleFieldToMatch::builder()
///                     .data("referer"). type ("HEADER").build_struct())
///                     .positionalConstraint("CONTAINS").targetString("badrefer1")
///                     .textTransformation("NONE").build_struct(),
///                 ],
///             )
///             .name("my_waf_byte_match_set")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Byte Match Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/byteMatchSet:ByteMatchSet byte_set a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod byte_match_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ByteMatchSetArgs {
        /// Settings for the ByteMatchSet, such as the bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests. ByteMatchTuple documented below.
        #[builder(into, default)]
        pub byte_match_tuples: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::ByteMatchSetByteMatchTuple>>,
        >,
        /// The name or description of the ByteMatchSet.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ByteMatchSetResult {
        /// Settings for the ByteMatchSet, such as the bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests. ByteMatchTuple documented below.
        pub byte_match_tuples: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::ByteMatchSetByteMatchTuple>>,
        >,
        /// The name or description of the ByteMatchSet.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ByteMatchSetArgs) -> ByteMatchSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let byte_match_tuples_binding = args.byte_match_tuples.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/byteMatchSet:ByteMatchSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "byteMatchTuples".into(),
                    value: &byte_match_tuples_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "byteMatchTuples".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ByteMatchSetResult {
            byte_match_tuples: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("byteMatchTuples").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
