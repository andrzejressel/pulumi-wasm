/// Provides a Cloudflare Waiting Room Rules resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = waiting_room_rules::create(
///         "example",
///         WaitingRoomRulesArgs::builder()
///             .rules(
///                 vec![
///                     WaitingRoomRulesRule::builder().action("bypass_waiting_room")
///                     .description("bypass ip list")
///                     .expression("src.ip in {192.0.2.0 192.0.2.1}").status("enabled")
///                     .build_struct(), WaitingRoomRulesRule::builder()
///                     .action("bypass_waiting_room").description("bypass query string")
///                     .expression("http.request.uri.query contains \"bypass=true\"")
///                     .status("enabled").build_struct(),
///                 ],
///             )
///             .waiting_room_id("d41d8cd98f00b204e9800998ecf8427e")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/waitingRoomRules:WaitingRoomRules default <zone_id>/<waiting_room_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod waiting_room_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WaitingRoomRulesArgs {
        /// List of rules to apply to the ruleset.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::WaitingRoomRulesRule>>,
        >,
        /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub waiting_room_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WaitingRoomRulesResult {
        /// List of rules to apply to the ruleset.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::WaitingRoomRulesRule>>,
        >,
        /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
        pub waiting_room_id: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WaitingRoomRulesArgs,
    ) -> WaitingRoomRulesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_output(context).get_inner();
        let waiting_room_id_binding = args
            .waiting_room_id
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomRules:WaitingRoomRules".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "waitingRoomId".into(),
                    value: &waiting_room_id_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WaitingRoomRulesResult {
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
            waiting_room_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("waitingRoomId"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
