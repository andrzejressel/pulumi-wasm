#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zero_trust_access_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZeroTrustAccessApplicationArgs {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The primary hostname and path that Access will secure. Must provide only one of `name`, `domain`.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the Access Application. Must provide only one of `name`, `domain`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZeroTrustAccessApplicationResult {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Application Audience (AUD) Tag of the application.
        pub aud: pulumi_gestalt_rust::Output<String>,
        /// The primary hostname and path that Access will secure. Must provide only one of `name`, `domain`.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Friendly name of the Access Application. Must provide only one of `name`, `domain`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZeroTrustAccessApplicationArgs,
    ) -> GetZeroTrustAccessApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let name_binding = args.name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getZeroTrustAccessApplication:getZeroTrustAccessApplication"
                .into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZeroTrustAccessApplicationResult {
            account_id: o.get_field("accountId"),
            aud: o.get_field("aud"),
            domain: o.get_field("domain"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
