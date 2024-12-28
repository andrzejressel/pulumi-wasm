/// Provides a Cloudflare Waiting Room Rules resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod waiting_room_rules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WaitingRoomRulesArgs {
        /// List of rules to apply to the ruleset.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WaitingRoomRulesRule>>,
        >,
        /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WaitingRoomRulesResult {
        /// List of rules to apply to the ruleset.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::types::WaitingRoomRulesRule>>,
        >,
        /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WaitingRoomRulesArgs) -> WaitingRoomRulesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_inner();
        let waiting_room_id_binding = args.waiting_room_id.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomRules:WaitingRoomRules".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "waitingRoomId".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WaitingRoomRulesResult {
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            waiting_room_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitingRoomId").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
