/// Provides an EC2 placement group. Read more about placement groups
/// in [AWS Docs](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod placement_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlacementGroupArgs {
        /// The name of the placement group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of partitions to create in the
        /// placement group.  Can only be specified when the `strategy` is set to
        /// `partition`.  Valid values are 1 - 7 (default is `2`).
        #[builder(into, default)]
        pub partition_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Determines how placement groups spread instances. Can only be used
        /// when the `strategy` is set to `spread`. Can be `host` or `rack`. `host` can only be used for Outpost placement groups. Defaults to `rack`.
        #[builder(into, default)]
        pub spread_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The placement strategy. Can be `cluster`, `partition` or `spread`.
        #[builder(into)]
        pub strategy: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlacementGroupResult {
        /// Amazon Resource Name (ARN) of the placement group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the placement group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of partitions to create in the
        /// placement group.  Can only be specified when the `strategy` is set to
        /// `partition`.  Valid values are 1 - 7 (default is `2`).
        pub partition_count: pulumi_wasm_rust::Output<i32>,
        /// The ID of the placement group.
        pub placement_group_id: pulumi_wasm_rust::Output<String>,
        /// Determines how placement groups spread instances. Can only be used
        /// when the `strategy` is set to `spread`. Can be `host` or `rack`. `host` can only be used for Outpost placement groups. Defaults to `rack`.
        pub spread_level: pulumi_wasm_rust::Output<String>,
        /// The placement strategy. Can be `cluster`, `partition` or `spread`.
        pub strategy: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PlacementGroupArgs) -> PlacementGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let partition_count_binding = args.partition_count.get_inner();
        let spread_level_binding = args.spread_level.get_inner();
        let strategy_binding = args.strategy.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/placementGroup:PlacementGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionCount".into(),
                    value: &partition_count_binding,
                },
                register_interface::ObjectField {
                    name: "spreadLevel".into(),
                    value: &spread_level_binding,
                },
                register_interface::ObjectField {
                    name: "strategy".into(),
                    value: &strategy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partitionCount".into(),
                },
                register_interface::ResultField {
                    name: "placementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "spreadLevel".into(),
                },
                register_interface::ResultField {
                    name: "strategy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PlacementGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partition_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionCount").unwrap(),
            ),
            placement_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementGroupId").unwrap(),
            ),
            spread_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spreadLevel").unwrap(),
            ),
            strategy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("strategy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}