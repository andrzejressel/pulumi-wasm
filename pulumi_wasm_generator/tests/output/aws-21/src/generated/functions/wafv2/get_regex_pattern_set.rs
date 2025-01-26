pub mod get_regex_pattern_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegexPatternSetArgs {
        /// Name of the WAFv2 Regex Pattern Set.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegexPatternSetResult {
        /// ARN of the entity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the set that helps with identification.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more blocks of regular expression patterns that AWS WAF is searching for. See Regular Expression below for details.
        pub regular_expressions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::wafv2::GetRegexPatternSetRegularExpression>,
        >,
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegexPatternSetArgs,
    ) -> GetRegexPatternSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:wafv2/getRegexPatternSet:getRegexPatternSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegexPatternSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            regular_expressions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("regularExpressions"),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
