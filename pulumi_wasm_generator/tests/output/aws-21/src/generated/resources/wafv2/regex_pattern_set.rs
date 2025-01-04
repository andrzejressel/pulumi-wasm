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
pub mod regex_pattern_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexPatternSetArgs {
        /// A friendly description of the regular expression pattern set.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A friendly name of the regular expression pattern set.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more blocks of regular expression patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`. See Regular Expression below for details. A maximum of 10 `regular_expression` blocks may be specified.
        #[builder(into, default)]
        pub regular_expressions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::RegexPatternSetRegularExpression>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegexPatternSetResult {
        /// The Amazon Resource Name (ARN) that identifies the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A friendly description of the regular expression pattern set.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub lock_token: pulumi_wasm_rust::Output<String>,
        /// A friendly name of the regular expression pattern set.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more blocks of regular expression patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`. See Regular Expression below for details. A maximum of 10 `regular_expression` blocks may be specified.
        pub regular_expressions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::RegexPatternSetRegularExpression>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegexPatternSetArgs) -> RegexPatternSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let regular_expressions_binding = args.regular_expressions.get_inner();
        let scope_binding = args.scope.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafv2/regexPatternSet:RegexPatternSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "regularExpressions".into(),
                    value: &regular_expressions_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lockToken".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "regularExpressions".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            lock_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockToken").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            regular_expressions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regularExpressions").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
