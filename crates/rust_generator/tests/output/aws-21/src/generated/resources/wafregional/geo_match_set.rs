/// Provides a WAF Regional Geo Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// Using `pulumi import`, import WAF Regional Geo Match Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/geoMatchSet:GeoMatchSet geo_match_set a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod geo_match_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GeoMatchSetArgs {
        /// The Geo Match Constraint objects which contain the country that you want AWS WAF to search for.
        #[builder(into, default)]
        pub geo_match_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafregional::GeoMatchSetGeoMatchConstraint>>,
        >,
        /// The name or description of the Geo Match Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GeoMatchSetResult {
        /// The Geo Match Constraint objects which contain the country that you want AWS WAF to search for.
        pub geo_match_constraints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafregional::GeoMatchSetGeoMatchConstraint>>,
        >,
        /// The name or description of the Geo Match Set.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GeoMatchSetArgs,
    ) -> GeoMatchSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let geo_match_constraints_binding_1 = args
            .geo_match_constraints
            .get_output(context);
        let geo_match_constraints_binding = geo_match_constraints_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/geoMatchSet:GeoMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        GeoMatchSetResult {
            geo_match_constraints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("geoMatchConstraints"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
