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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CostAllocationTagArgs,
    ) -> CostAllocationTagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let status_binding = args.status.get_output(context);
        let tag_key_binding = args.tag_key.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:costexplorer/costAllocationTag:CostAllocationTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagKey".into(),
                    value: tag_key_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CostAllocationTagResult {
            status: o.get_field("status"),
            tag_key: o.get_field("tagKey"),
            type_: o.get_field("type"),
        }
    }
}
