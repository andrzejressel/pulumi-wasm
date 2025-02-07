/// Provides a resource to manage API Shield configurations.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldArgs {
        /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
        #[builder(into, default)]
        pub auth_id_characteristics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldResult {
        /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
        pub auth_id_characteristics: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
        >,
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
        args: ApiShieldArgs,
    ) -> ApiShieldResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auth_id_characteristics_binding = args
            .auth_id_characteristics
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/apiShield:ApiShield".into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiShieldResult {
            auth_id_characteristics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authIdCharacteristics"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
