/// Provides a Cloudflare Cache Reserve resource. Cache Reserve can
/// increase cache lifetimes by automatically storing all cacheable
/// files in Cloudflare's persistent object storage buckets.
///
/// Note: Using Cache Reserve without Tiered Cache is not recommended.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone_cache_reserve::create(
///         "example",
///         ZoneCacheReserveArgs::builder()
///             .enabled(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zoneCacheReserve:ZoneCacheReserve example <zone_id>
/// ```
///
pub mod zone_cache_reserve {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneCacheReserveArgs {
        /// Whether to enable or disable Cache Reserve support for a given zone.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneCacheReserveResult {
        /// Whether to enable or disable Cache Reserve support for a given zone.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZoneCacheReserveArgs,
    ) -> ZoneCacheReserveResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheReserve:ZoneCacheReserve".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZoneCacheReserveResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
