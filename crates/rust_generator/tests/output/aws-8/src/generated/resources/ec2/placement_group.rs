/// Provides an EC2 placement group. Read more about placement groups
/// in [AWS Docs](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let web = placement_group::create(
///         "web",
///         PlacementGroupArgs::builder()
///             .name("hunky-dory-pg")
///             .strategy("cluster")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import placement groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/placementGroup:PlacementGroup prod_pg production-placement-group
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod placement_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlacementGroupArgs {
        /// The name of the placement group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of partitions to create in the
        /// placement group.  Can only be specified when the `strategy` is set to
        /// `partition`.  Valid values are 1 - 7 (default is `2`).
        #[builder(into, default)]
        pub partition_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Determines how placement groups spread instances. Can only be used
        /// when the `strategy` is set to `spread`. Can be `host` or `rack`. `host` can only be used for Outpost placement groups. Defaults to `rack`.
        #[builder(into, default)]
        pub spread_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The placement strategy. Can be `cluster`, `partition` or `spread`.
        #[builder(into)]
        pub strategy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlacementGroupResult {
        /// Amazon Resource Name (ARN) of the placement group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the placement group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of partitions to create in the
        /// placement group.  Can only be specified when the `strategy` is set to
        /// `partition`.  Valid values are 1 - 7 (default is `2`).
        pub partition_count: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the placement group.
        pub placement_group_id: pulumi_gestalt_rust::Output<String>,
        /// Determines how placement groups spread instances. Can only be used
        /// when the `strategy` is set to `spread`. Can be `host` or `rack`. `host` can only be used for Outpost placement groups. Defaults to `rack`.
        pub spread_level: pulumi_gestalt_rust::Output<String>,
        /// The placement strategy. Can be `cluster`, `partition` or `spread`.
        pub strategy: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: PlacementGroupArgs,
    ) -> PlacementGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let partition_count_binding = args.partition_count.get_output(context);
        let spread_level_binding = args.spread_level.get_output(context);
        let strategy_binding = args.strategy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/placementGroup:PlacementGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionCount".into(),
                    value: &partition_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spreadLevel".into(),
                    value: &spread_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "strategy".into(),
                    value: &strategy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PlacementGroupResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            partition_count: o.get_field("partitionCount"),
            placement_group_id: o.get_field("placementGroupId"),
            spread_level: o.get_field("spreadLevel"),
            strategy: o.get_field("strategy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
