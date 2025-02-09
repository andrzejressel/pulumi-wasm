/// Provides an AWS WAFv2 Regex Pattern Set Resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:wafv2:RegexPatternSet
///     properties:
///       name: example
///       description: Example regex pattern set
///       scope: REGIONAL
///       regularExpressions:
///         - regexString: one
///         - regexString: two
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAFv2 Regex Pattern Sets using `ID/name/scope`. For example:
///
/// ```sh
/// $ pulumi import aws:wafv2/regexPatternSet:RegexPatternSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc/example/REGIONAL
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regex_pattern_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexPatternSetArgs {
        /// A friendly description of the regular expression pattern set.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A friendly name of the regular expression pattern set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more blocks of regular expression patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`. See Regular Expression below for details. A maximum of 10 `regular_expression` blocks may be specified.
        #[builder(into, default)]
        pub regular_expressions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafv2::RegexPatternSetRegularExpression>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegexPatternSetResult {
        /// The Amazon Resource Name (ARN) that identifies the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A friendly description of the regular expression pattern set.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub lock_token: pulumi_gestalt_rust::Output<String>,
        /// A friendly name of the regular expression pattern set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more blocks of regular expression patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`. See Regular Expression below for details. A maximum of 10 `regular_expression` blocks may be specified.
        pub regular_expressions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafv2::RegexPatternSetRegularExpression>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegexPatternSetArgs,
    ) -> RegexPatternSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let regular_expressions_binding = args.regular_expressions.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafv2/regexPatternSet:RegexPatternSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regularExpressions".into(),
                    value: regular_expressions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegexPatternSetResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            lock_token: o.get_field("lockToken"),
            name: o.get_field("name"),
            regular_expressions: o.get_field("regularExpressions"),
            scope: o.get_field("scope"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
