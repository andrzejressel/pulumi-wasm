pub mod get_agreement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAgreementArgs {
        /// The Offer of the Marketplace Image.
        #[builder(into)]
        pub offer: pulumi_wasm_rust::Output<String>,
        /// The Plan of the Marketplace Image.
        #[builder(into)]
        pub plan: pulumi_wasm_rust::Output<String>,
        /// The Publisher of the Marketplace Image.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAgreementResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub license_text_link: pulumi_wasm_rust::Output<String>,
        pub offer: pulumi_wasm_rust::Output<String>,
        pub plan: pulumi_wasm_rust::Output<String>,
        pub privacy_policy_link: pulumi_wasm_rust::Output<String>,
        pub publisher: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAgreementArgs) -> GetAgreementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let offer_binding = args.offer.get_inner();
        let plan_binding = args.plan.get_inner();
        let publisher_binding = args.publisher.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:marketplace/getAgreement:getAgreement".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "offer".into(),
                    value: &offer_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "publisher".into(),
                    value: &publisher_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "licenseTextLink".into(),
                },
                register_interface::ResultField {
                    name: "offer".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "privacyPolicyLink".into(),
                },
                register_interface::ResultField {
                    name: "publisher".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAgreementResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            license_text_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseTextLink").unwrap(),
            ),
            offer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offer").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            privacy_policy_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privacyPolicyLink").unwrap(),
            ),
            publisher: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisher").unwrap(),
            ),
        }
    }
}
