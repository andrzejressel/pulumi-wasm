#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneArgs {
        /// The account identifier to target for the resource.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the zone. Must provide only one of `zone_id`, `name`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZoneResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the zone. Must provide only one of `zone_id`, `name`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Cloudflare assigned name servers. This is only populated for zones that use Cloudflare DNS.
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether the zone is paused on Cloudflare.
        pub paused: pulumi_gestalt_rust::Output<bool>,
        /// The name of the plan associated with the zone.
        pub plan: pulumi_gestalt_rust::Output<String>,
        /// Status of the zone.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// List of Vanity Nameservers (if set).
        pub vanity_name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZoneArgs,
    ) -> GetZoneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getZone:getZone".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZoneResult {
            account_id: o.get_field("accountId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            name_servers: o.get_field("nameServers"),
            paused: o.get_field("paused"),
            plan: o.get_field("plan"),
            status: o.get_field("status"),
            vanity_name_servers: o.get_field("vanityNameServers"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
