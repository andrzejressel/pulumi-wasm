pub mod get_traffic_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrafficPolicyDocumentArgs {
        /// Configuration block for the definitions of the endpoints that you want to use in this traffic policy. See below
        #[builder(into, default)]
        pub endpoints: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::route53::GetTrafficPolicyDocumentEndpoint,
                >,
            >,
        >,
        /// DNS type of all of the resource record sets that Amazon Route 53 will create based on this traffic policy.
        #[builder(into, default)]
        pub record_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for definitions of the rules that you want to use in this traffic policy. See below
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::route53::GetTrafficPolicyDocumentRule>,
            >,
        >,
        /// An endpoint to be as the starting point for the traffic policy.
        #[builder(into, default)]
        pub start_endpoint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A rule to be as the starting point for the traffic policy.
        #[builder(into, default)]
        pub start_rule: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Version of the traffic policy format.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTrafficPolicyDocumentResult {
        pub endpoints: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::route53::GetTrafficPolicyDocumentEndpoint,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_wasm_rust::Output<String>,
        pub record_type: pulumi_wasm_rust::Output<Option<String>>,
        pub rules: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::route53::GetTrafficPolicyDocumentRule>,
            >,
        >,
        pub start_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        pub start_rule: pulumi_wasm_rust::Output<Option<String>>,
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTrafficPolicyDocumentArgs,
    ) -> GetTrafficPolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoints_binding = args.endpoints.get_output(context).get_inner();
        let record_type_binding = args.record_type.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let start_endpoint_binding = args.start_endpoint.get_output(context).get_inner();
        let start_rule_binding = args.start_rule.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getTrafficPolicyDocument:getTrafficPolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpoints".into(),
                    value: &endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "recordType".into(),
                    value: &record_type_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "startEndpoint".into(),
                    value: &start_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "startRule".into(),
                    value: &start_rule_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTrafficPolicyDocumentResult {
            endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoints"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            json: pulumi_wasm_rust::__private::into_domain(o.extract_field("json")),
            record_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recordType"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            start_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startEndpoint"),
            ),
            start_rule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startRule"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
