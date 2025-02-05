/// Provides a Cloudflare Zone Hold resource that prevents adding
/// the hostname to another account for use.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone_hold::create(
///         "example",
///         ZoneHoldArgs::builder()
///             .hold(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zoneHold:ZoneHold example <zone_id>
/// ```
///
pub mod zone_hold {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneHoldArgs {
        /// Enablement status of the zone hold.
        #[builder(into)]
        pub hold: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
        #[builder(into, default)]
        pub hold_after: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to extend to block any subdomain of the given zone.
        #[builder(into, default)]
        pub include_subdomains: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneHoldResult {
        /// Enablement status of the zone hold.
        pub hold: pulumi_wasm_rust::Output<bool>,
        /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
        pub hold_after: pulumi_wasm_rust::Output<String>,
        /// Whether to extend to block any subdomain of the given zone.
        pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZoneHoldArgs,
    ) -> ZoneHoldResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hold_binding = args.hold.get_output(context).get_inner();
        let hold_after_binding = args.hold_after.get_output(context).get_inner();
        let include_subdomains_binding = args
            .include_subdomains
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zoneHold:ZoneHold".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hold".into(),
                    value: &hold_binding,
                },
                register_interface::ObjectField {
                    name: "holdAfter".into(),
                    value: &hold_after_binding,
                },
                register_interface::ObjectField {
                    name: "includeSubdomains".into(),
                    value: &include_subdomains_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZoneHoldResult {
            hold: pulumi_wasm_rust::__private::into_domain(o.extract_field("hold")),
            hold_after: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("holdAfter"),
            ),
            include_subdomains: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeSubdomains"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
