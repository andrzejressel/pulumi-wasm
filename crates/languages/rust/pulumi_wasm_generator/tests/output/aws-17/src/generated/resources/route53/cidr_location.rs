/// Provides a Route53 CIDR location resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cidr_collection::create(
///         "example",
///         CidrCollectionArgs::builder().name("collection-1").build_struct(),
///     );
///     let exampleCidrLocation = cidr_location::create(
///         "exampleCidrLocation",
///         CidrLocationArgs::builder()
///             .cidr_blocks(vec!["200.5.3.0/24", "200.6.3.0/24",])
///             .cidr_collection_id("${example.id}")
///             .name("office")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CIDR locations using their the CIDR collection ID and location name. For example:
///
/// ```sh
/// $ pulumi import aws:route53/cidrLocation:CidrLocation example 9ac32814-3e67-0932-6048-8d779cc6f511,office
/// ```
pub mod cidr_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CidrLocationArgs {
        /// CIDR blocks for the location.
        #[builder(into)]
        pub cidr_blocks: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The ID of the CIDR collection to update.
        #[builder(into)]
        pub cidr_collection_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name for the CIDR location.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CidrLocationResult {
        /// CIDR blocks for the location.
        pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the CIDR collection to update.
        pub cidr_collection_id: pulumi_wasm_rust::Output<String>,
        /// Name for the CIDR location.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CidrLocationArgs,
    ) -> CidrLocationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_blocks_binding = args.cidr_blocks.get_output(context).get_inner();
        let cidr_collection_id_binding = args
            .cidr_collection_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/cidrLocation:CidrLocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrBlocks".into(),
                    value: &cidr_blocks_binding,
                },
                register_interface::ObjectField {
                    name: "cidrCollectionId".into(),
                    value: &cidr_collection_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CidrLocationResult {
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrBlocks"),
            ),
            cidr_collection_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cidrCollectionId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
