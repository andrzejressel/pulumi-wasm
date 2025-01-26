pub mod get_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneArgs {
        /// The account identifier to target for the resource.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the zone. Must provide only one of `zone_id`, `name`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZoneResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The name of the zone. Must provide only one of `zone_id`, `name`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Cloudflare assigned name servers. This is only populated for zones that use Cloudflare DNS.
        pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether the zone is paused on Cloudflare.
        pub paused: pulumi_wasm_rust::Output<bool>,
        /// The name of the plan associated with the zone.
        pub plan: pulumi_wasm_rust::Output<String>,
        /// Status of the zone.
        pub status: pulumi_wasm_rust::Output<String>,
        /// List of Vanity Nameservers (if set).
        pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetZoneArgs,
    ) -> GetZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getZone:getZone".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZoneResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            name_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nameServers"),
            ),
            paused: pulumi_wasm_rust::__private::into_domain(o.extract_field("paused")),
            plan: pulumi_wasm_rust::__private::into_domain(o.extract_field("plan")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            vanity_name_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vanityNameServers"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
