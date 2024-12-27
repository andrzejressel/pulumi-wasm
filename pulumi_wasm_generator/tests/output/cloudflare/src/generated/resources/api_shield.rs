/// Provides a resource to manage API Shield configurations.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api_shield::create(
///         "example",
///         ApiShieldArgs::builder()
///             .auth_id_characteristics(
///                 vec![
///                     ApiShieldAuthIdCharacteristic::builder().name("my-example-header").
///                     type ("header").build_struct(),
///                 ],
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
pub mod api_shield {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldArgs {
        /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
        #[builder(into, default)]
        pub auth_id_characteristics: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldResult {
        /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
        pub auth_id_characteristics: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiShieldArgs) -> ApiShieldResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auth_id_characteristics_binding = args.auth_id_characteristics.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/apiShield:ApiShield".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authIdCharacteristics".into(),
                    value: &auth_id_characteristics_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authIdCharacteristics".into(),
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
        ApiShieldResult {
            auth_id_characteristics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authIdCharacteristics").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
