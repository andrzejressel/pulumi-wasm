/// The [Risk Behavior](https://developers.cloudflare.com/cloudflare-one/insights/risk-score/) resource allows you to configure Cloudflare Risk Behaviors for an account.
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod risk_behavior {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RiskBehaviorArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Zero Trust risk behaviors configured on this account
        #[builder(into, default)]
        pub behaviors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::RiskBehaviorBehavior>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RiskBehaviorResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Zero Trust risk behaviors configured on this account
        pub behaviors: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::RiskBehaviorBehavior>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RiskBehaviorArgs,
    ) -> RiskBehaviorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let behaviors_binding = args.behaviors.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/riskBehavior:RiskBehavior".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "behaviors".into(),
                    value: behaviors_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RiskBehaviorResult {
            account_id: o.get_field("accountId"),
            behaviors: o.get_field("behaviors"),
        }
    }
}
