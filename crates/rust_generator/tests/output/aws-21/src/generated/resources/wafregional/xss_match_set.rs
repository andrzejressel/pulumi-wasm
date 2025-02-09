/// Provides a WAF Regional XSS Match Set Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod xss_match_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct XssMatchSetArgs {
        /// The name of the set
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parts of web requests that you want to inspect for cross-site scripting attacks.
        #[builder(into, default)]
        pub xss_match_tuples: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafregional::XssMatchSetXssMatchTuple>>,
        >,
    }
    #[allow(dead_code)]
    pub struct XssMatchSetResult {
        /// The name of the set
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parts of web requests that you want to inspect for cross-site scripting attacks.
        pub xss_match_tuples: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafregional::XssMatchSetXssMatchTuple>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: XssMatchSetArgs,
    ) -> XssMatchSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let xss_match_tuples_binding = args.xss_match_tuples.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/xssMatchSet:XssMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xssMatchTuples".into(),
                    value: xss_match_tuples_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        XssMatchSetResult {
            name: o.get_field("name"),
            xss_match_tuples: o.get_field("xssMatchTuples"),
        }
    }
}
