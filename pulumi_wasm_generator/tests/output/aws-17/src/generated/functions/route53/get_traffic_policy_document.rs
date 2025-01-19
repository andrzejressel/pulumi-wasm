pub mod get_traffic_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrafficPolicyDocumentArgs {
        /// Configuration block for the definitions of the endpoints that you want to use in this traffic policy. See below
        #[builder(into, default)]
        pub endpoints: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::route53::GetTrafficPolicyDocumentEndpoint,
                >,
            >,
        >,
        /// DNS type of all of the resource record sets that Amazon Route 53 will create based on this traffic policy.
        #[builder(into, default)]
        pub record_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for definitions of the rules that you want to use in this traffic policy. See below
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::route53::GetTrafficPolicyDocumentRule>,
            >,
        >,
        /// An endpoint to be as the starting point for the traffic policy.
        #[builder(into, default)]
        pub start_endpoint: pulumi_wasm_rust::Output<Option<String>>,
        /// A rule to be as the starting point for the traffic policy.
        #[builder(into, default)]
        pub start_rule: pulumi_wasm_rust::Output<Option<String>>,
        /// Version of the traffic policy format.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetTrafficPolicyDocumentArgs) -> GetTrafficPolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoints_binding = args.endpoints.get_inner();
        let record_type_binding = args.record_type.get_inner();
        let rules_binding = args.rules.get_inner();
        let start_endpoint_binding = args.start_endpoint.get_inner();
        let start_rule_binding = args.start_rule.get_inner();
        let version_binding = args.version.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "json".into(),
                },
                register_interface::ResultField {
                    name: "recordType".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "startEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "startRule".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTrafficPolicyDocumentResult {
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("json").unwrap(),
            ),
            record_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recordType").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            start_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startEndpoint").unwrap(),
            ),
            start_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startRule").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
