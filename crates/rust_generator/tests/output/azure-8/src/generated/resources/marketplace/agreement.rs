/// Allows accepting the Legal Terms for a Marketplace Image.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agreement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgreementArgs {
        /// The Offer of the Marketplace Image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub offer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Plan of the Marketplace Image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Publisher of the Marketplace Image. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AgreementResult {
        pub license_text_link: pulumi_gestalt_rust::Output<String>,
        /// The Offer of the Marketplace Image. Changing this forces a new resource to be created.
        pub offer: pulumi_gestalt_rust::Output<String>,
        /// The Plan of the Marketplace Image. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<String>,
        pub privacy_policy_link: pulumi_gestalt_rust::Output<String>,
        /// The Publisher of the Marketplace Image. Changing this forces a new resource to be created.
        pub publisher: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AgreementArgs,
    ) -> AgreementResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let offer_binding = args.offer.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let publisher_binding = args.publisher.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:marketplace/agreement:Agreement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offer".into(),
                    value: offer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisher".into(),
                    value: publisher_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AgreementResult {
            license_text_link: o.get_field("licenseTextLink"),
            offer: o.get_field("offer"),
            plan: o.get_field("plan"),
            privacy_policy_link: o.get_field("privacyPolicyLink"),
            publisher: o.get_field("publisher"),
        }
    }
}
