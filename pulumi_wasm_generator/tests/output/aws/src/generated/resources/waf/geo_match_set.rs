/// Provides a WAF Geo Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let geoMatchSet = geo_match_set::create(
///         "geoMatchSet",
///         GeoMatchSetArgs::builder()
///             .geo_match_constraints(
///                 vec![
///                     GeoMatchSetGeoMatchConstraint::builder(). type ("Country")
///                     .value("US").build_struct(), GeoMatchSetGeoMatchConstraint::builder()
///                     . type ("Country").value("CA").build_struct(),
///                 ],
///             )
///             .name("geo_match_set")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Geo Match Set using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:waf/geoMatchSet:GeoMatchSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod geo_match_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GeoMatchSetArgs {
        /// The GeoMatchConstraint objects which contain the country that you want AWS WAF to search for.
        #[builder(into, default)]
        pub geo_match_constraints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::GeoMatchSetGeoMatchConstraint>>,
        >,
        /// The name or description of the GeoMatchSet.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GeoMatchSetResult {
        /// Amazon Resource Name (ARN)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The GeoMatchConstraint objects which contain the country that you want AWS WAF to search for.
        pub geo_match_constraints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::GeoMatchSetGeoMatchConstraint>>,
        >,
        /// The name or description of the GeoMatchSet.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GeoMatchSetArgs) -> GeoMatchSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let geo_match_constraints_binding = args.geo_match_constraints.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:waf/geoMatchSet:GeoMatchSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "geoMatchConstraints".into(),
                    value: &geo_match_constraints_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "geoMatchConstraints".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GeoMatchSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            geo_match_constraints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoMatchConstraints").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
