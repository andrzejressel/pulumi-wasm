/// Allows accepting the Legal Terms for a Marketplace Image.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let barracuda = agreement::create(
///         "barracuda",
///         AgreementArgs::builder()
///             .offer("waf")
///             .plan("hourly")
///             .publisher("barracudanetworks")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Marketplace Agreement can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:marketplace/agreement:Agreement example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.MarketplaceOrdering/agreements/publisher1/offers/offer1/plans/plan1
/// ```
///
pub mod agreement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgreementArgs {
        /// The Offer of the Marketplace Image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub offer: pulumi_wasm_rust::Output<String>,
        /// The Plan of the Marketplace Image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub plan: pulumi_wasm_rust::Output<String>,
        /// The Publisher of the Marketplace Image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AgreementResult {
        pub license_text_link: pulumi_wasm_rust::Output<String>,
        /// The Offer of the Marketplace Image. Changing this forces a new resource to be created.
        pub offer: pulumi_wasm_rust::Output<String>,
        /// The Plan of the Marketplace Image. Changing this forces a new resource to be created.
        pub plan: pulumi_wasm_rust::Output<String>,
        pub privacy_policy_link: pulumi_wasm_rust::Output<String>,
        /// The Publisher of the Marketplace Image. Changing this forces a new resource to be created.
        pub publisher: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AgreementArgs) -> AgreementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let offer_binding = args.offer.get_inner();
        let plan_binding = args.plan.get_inner();
        let publisher_binding = args.publisher.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:marketplace/agreement:Agreement".into(),
            name: name.to_string(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AgreementResult {
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