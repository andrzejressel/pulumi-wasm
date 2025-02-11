#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_agreement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAgreementArgs {
        /// The Offer of the Marketplace Image.
        #[builder(into)]
        pub offer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Plan of the Marketplace Image.
        #[builder(into)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Publisher of the Marketplace Image.
        #[builder(into)]
        pub publisher: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAgreementResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub license_text_link: pulumi_gestalt_rust::Output<String>,
        pub offer: pulumi_gestalt_rust::Output<String>,
        pub plan: pulumi_gestalt_rust::Output<String>,
        pub privacy_policy_link: pulumi_gestalt_rust::Output<String>,
        pub publisher: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAgreementArgs,
    ) -> GetAgreementResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let offer_binding = args.offer.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let publisher_binding = args.publisher.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:marketplace/getAgreement:getAgreement".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offer".into(),
                    value: &offer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisher".into(),
                    value: &publisher_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAgreementResult {
            id: o.get_field("id"),
            license_text_link: o.get_field("licenseTextLink"),
            offer: o.get_field("offer"),
            plan: o.get_field("plan"),
            privacy_policy_link: o.get_field("privacyPolicyLink"),
            publisher: o.get_field("publisher"),
        }
    }
}
