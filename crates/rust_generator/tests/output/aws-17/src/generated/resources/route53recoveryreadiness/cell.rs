/// Provides an AWS Route 53 Recovery Readiness Cell.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cell::create(
///         "example",
///         CellArgs::builder().cell_name("us-west-2-failover-cell").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Readiness cells using the cell name. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoveryreadiness/cell:Cell us-west-2-failover-cell us-west-2-failover-cell
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cell {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CellArgs {
        /// Unique name describing the cell.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub cell_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of cell arns to add as nested fault domains within this cell.
        #[builder(into, default)]
        pub cells: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CellResult {
        /// ARN of the cell
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unique name describing the cell.
        ///
        /// The following arguments are optional:
        pub cell_name: pulumi_gestalt_rust::Output<String>,
        /// List of cell arns to add as nested fault domains within this cell.
        pub cells: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of readiness scopes (recovery groups or cells) that contain this cell.
        pub parent_readiness_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CellArgs,
    ) -> CellResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cell_name_binding = args.cell_name.get_output(context);
        let cells_binding = args.cells.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53recoveryreadiness/cell:Cell".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cellName".into(),
                    value: &cell_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cells".into(),
                    value: &cells_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CellResult {
            arn: o.get_field("arn"),
            cell_name: o.get_field("cellName"),
            cells: o.get_field("cells"),
            parent_readiness_scopes: o.get_field("parentReadinessScopes"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
