pub mod get_agreement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAgreementArgs {
        /// The Offer of the Marketplace Image.
        #[builder(into)]
        pub offer: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Plan of the Marketplace Image.
        #[builder(into)]
        pub plan: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Publisher of the Marketplace Image.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAgreementArgs,
    ) -> GetAgreementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let offer_binding = args.offer.get_output(context).get_inner();
        let plan_binding = args.plan.get_output(context).get_inner();
        let publisher_binding = args.publisher.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:marketplace/getAgreement:getAgreement".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAgreementResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            license_text_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenseTextLink"),
            ),
            offer: pulumi_wasm_rust::__private::into_domain(o.extract_field("offer")),
            plan: pulumi_wasm_rust::__private::into_domain(o.extract_field("plan")),
            privacy_policy_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privacyPolicyLink"),
            ),
            publisher: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publisher"),
            ),
        }
    }
}
