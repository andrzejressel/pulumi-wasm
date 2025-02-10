/// Provides a WAF Byte Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// Using `pulumi import`, import WAF Byte Match Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:waf/byteMatchSet:ByteMatchSet byte_set a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod byte_match_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ByteMatchSetArgs {
        /// Specifies the bytes (typically a string that corresponds
        /// with ASCII characters) that you want to search for in web requests,
        /// the location in requests that you want to search, and other settings.
        #[builder(into, default)]
        pub byte_match_tuples: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::waf::ByteMatchSetByteMatchTuple>>,
        >,
        /// The name or description of the Byte Match Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ByteMatchSetResult {
        /// Specifies the bytes (typically a string that corresponds
        /// with ASCII characters) that you want to search for in web requests,
        /// the location in requests that you want to search, and other settings.
        pub byte_match_tuples: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::waf::ByteMatchSetByteMatchTuple>>,
        >,
        /// The name or description of the Byte Match Set.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ByteMatchSetArgs,
    ) -> ByteMatchSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let byte_match_tuples_binding = args.byte_match_tuples.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:waf/byteMatchSet:ByteMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "byteMatchTuples".into(),
                    value: byte_match_tuples_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ByteMatchSetResult {
            byte_match_tuples: o.get_field("byteMatchTuples"),
            name: o.get_field("name"),
        }
    }
}
