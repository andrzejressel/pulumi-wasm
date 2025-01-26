/// Provides a CE Cost Allocation Tag.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cost_allocation_tag::create(
///         "example",
///         CostAllocationTagArgs::builder()
///             .status("Active")
///             .tag_key("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ce_cost_allocation_tag` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:costexplorer/costAllocationTag:CostAllocationTag example key
/// ```
pub mod cost_allocation_tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CostAllocationTagArgs {
        /// The status of a cost allocation tag. Valid values are `Active` and `Inactive`.
        #[builder(into)]
        pub status: pulumi_wasm_rust::InputOrOutput<String>,
        /// The key for the cost allocation tag.
        #[builder(into)]
        pub tag_key: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CostAllocationTagResult {
        /// The status of a cost allocation tag. Valid values are `Active` and `Inactive`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The key for the cost allocation tag.
        pub tag_key: pulumi_wasm_rust::Output<String>,
        /// The type of cost allocation tag.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CostAllocationTagArgs,
    ) -> CostAllocationTagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let status_binding = args.status.get_output(context).get_inner();
        let tag_key_binding = args.tag_key.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:costexplorer/costAllocationTag:CostAllocationTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tagKey".into(),
                    value: &tag_key_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tagKey".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CostAllocationTagResult {
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tag_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagKey").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
