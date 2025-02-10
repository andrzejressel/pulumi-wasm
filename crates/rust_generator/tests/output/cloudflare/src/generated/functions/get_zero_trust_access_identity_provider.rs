#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zero_trust_access_identity_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZeroTrustAccessIdentityProviderArgs {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Access Identity Provider name to search for.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZeroTrustAccessIdentityProviderResult {
        /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Access Identity Provider name to search for.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Access Identity Provider Type.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZeroTrustAccessIdentityProviderArgs,
    ) -> GetZeroTrustAccessIdentityProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getZeroTrustAccessIdentityProvider:getZeroTrustAccessIdentityProvider"
                .into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
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
        GetZeroTrustAccessIdentityProviderResult {
            account_id: o.get_field("accountId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
