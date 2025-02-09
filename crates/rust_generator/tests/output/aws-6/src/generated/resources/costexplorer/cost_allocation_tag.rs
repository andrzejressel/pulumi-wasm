/// Provides a CE Cost Allocation Tag.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cost_allocation_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CostAllocationTagArgs {
        /// The status of a cost allocation tag. Valid values are `Active` and `Inactive`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The key for the cost allocation tag.
        #[builder(into)]
        pub tag_key: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CostAllocationTagResult {
        /// The status of a cost allocation tag. Valid values are `Active` and `Inactive`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The key for the cost allocation tag.
        pub tag_key: pulumi_gestalt_rust::Output<String>,
        /// The type of cost allocation tag.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CostAllocationTagArgs,
    ) -> CostAllocationTagResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let status_binding_1 = args.status.get_output(context);
        let status_binding = status_binding_1.get_inner();
        let tag_key_binding_1 = args.tag_key.get_output(context);
        let tag_key_binding = tag_key_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        CostAllocationTagResult {
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tag_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagKey"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
