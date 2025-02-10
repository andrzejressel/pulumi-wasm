/// Provides a resource to manage a Resource Explorer index in the current AWS Region.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = index::create(
///         "example",
///         IndexArgs::builder().type_("LOCAL").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Resource Explorer indexes using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:resourceexplorer/index:Index example arn:aws:resource-explorer-2:us-east-1:123456789012:index/6047ac4e-207e-4487-9bcf-cb53bb0ff5cc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IndexArgs {
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::resourceexplorer::IndexTimeouts>,
        >,
        /// The type of the index. Valid values: `AGGREGATOR`, `LOCAL`. To understand the difference between `LOCAL` and `AGGREGATOR`, see the [_AWS Resource Explorer User Guide_](https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region.html).
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IndexResult {
        /// Amazon Resource Name (ARN) of the Resource Explorer index.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::resourceexplorer::IndexTimeouts>,
        >,
        /// The type of the index. Valid values: `AGGREGATOR`, `LOCAL`. To understand the difference between `LOCAL` and `AGGREGATOR`, see the [_AWS Resource Explorer User Guide_](https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region.html).
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IndexArgs,
    ) -> IndexResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:resourceexplorer/index:Index".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IndexResult {
            arn: o.get_field("arn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
        }
    }
}
