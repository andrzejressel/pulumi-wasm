/// Provides a Glue Data Quality Ruleset Resource. You can refer to the [Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/glue-data-quality.html) for a full explanation of the Glue Data Quality Ruleset functionality
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_quality_ruleset::create(
///         "example",
///         DataQualityRulesetArgs::builder()
///             .name("example")
///             .ruleset("Rules = [Completeness \"colA\" between 0.4 and 0.8]")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With description
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_quality_ruleset::create(
///         "example",
///         DataQualityRulesetArgs::builder()
///             .description("example")
///             .name("example")
///             .ruleset("Rules = [Completeness \"colA\" between 0.4 and 0.8]")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With tags
///
/// ```yaml
/// resources:
///   example:
///     type: aws:glue:DataQualityRuleset
///     properties:
///       name: example
///       ruleset: Rules = [Completeness "colA" between 0.4 and 0.8]
///       tags:
///         hello: world
/// ```
///
/// ### With target_table
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_quality_ruleset::create(
///         "example",
///         DataQualityRulesetArgs::builder()
///             .name("example")
///             .ruleset("Rules = [Completeness \"colA\" between 0.4 and 0.8]")
///             .target_table(
///                 DataQualityRulesetTargetTable::builder()
///                     .databaseName("${exampleAwsGlueCatalogDatabase.name}")
///                     .tableName("${exampleAwsGlueCatalogTable.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Data Quality Ruleset using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/dataQualityRuleset:DataQualityRuleset example exampleName
/// ```
pub mod data_quality_ruleset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataQualityRulesetArgs {
        /// Description of the data quality ruleset.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the data quality ruleset.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A Data Quality Definition Language (DQDL) ruleset. For more information, see the AWS Glue developer guide.
        #[builder(into)]
        pub ruleset: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A Configuration block specifying a target table associated with the data quality ruleset. See `target_table` below.
        #[builder(into, default)]
        pub target_table: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::DataQualityRulesetTargetTable>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataQualityRulesetResult {
        /// ARN of the Glue Data Quality Ruleset.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The time and date that this data quality ruleset was created.
        pub created_on: pulumi_wasm_rust::Output<String>,
        /// Description of the data quality ruleset.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The time and date that this data quality ruleset was created.
        pub last_modified_on: pulumi_wasm_rust::Output<String>,
        /// Name of the data quality ruleset.
        pub name: pulumi_wasm_rust::Output<String>,
        /// When a ruleset was created from a recommendation run, this run ID is generated to link the two together.
        pub recommendation_run_id: pulumi_wasm_rust::Output<String>,
        /// A Data Quality Definition Language (DQDL) ruleset. For more information, see the AWS Glue developer guide.
        pub ruleset: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A Configuration block specifying a target table associated with the data quality ruleset. See `target_table` below.
        pub target_table: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::DataQualityRulesetTargetTable>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataQualityRulesetArgs) -> DataQualityRulesetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let ruleset_binding = args.ruleset.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_table_binding = args.target_table.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/dataQualityRuleset:DataQualityRuleset".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ruleset".into(),
                    value: &ruleset_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetTable".into(),
                    value: &target_table_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recommendationRunId".into(),
                },
                register_interface::ResultField {
                    name: "ruleset".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "targetTable".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataQualityRulesetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            last_modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedOn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recommendation_run_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recommendationRunId").unwrap(),
            ),
            ruleset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleset").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            target_table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetTable").unwrap(),
            ),
        }
    }
}
