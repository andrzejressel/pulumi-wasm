/// Manages a Glue Crawler. More information can be found in the [AWS Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/add-crawler.html)
///
/// ## Example Usage
///
/// ### DynamoDB Target Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = crawler::create(
///         "example",
///         CrawlerArgs::builder()
///             .database_name("${exampleAwsGlueCatalogDatabase.name}")
///             .dynamodb_targets(
///                 vec![CrawlerDynamodbTarget::builder().path("table-name").build_struct(),],
///             )
///             .name("example")
///             .role("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### JDBC Target Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = crawler::create(
///         "example",
///         CrawlerArgs::builder()
///             .database_name("${exampleAwsGlueCatalogDatabase.name}")
///             .jdbc_targets(
///                 vec![
///                     CrawlerJdbcTarget::builder()
///                     .connectionName("${exampleAwsGlueConnection.name}")
///                     .path("database-name/%").build_struct(),
///                 ],
///             )
///             .name("example")
///             .role("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### S3 Target Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = crawler::create(
///         "example",
///         CrawlerArgs::builder()
///             .database_name("${exampleAwsGlueCatalogDatabase.name}")
///             .name("example")
///             .role("${exampleAwsIamRole.arn}")
///             .s_3_targets(
///                 vec![
///                     CrawlerS3Target::builder().path("s3://${exampleAwsS3Bucket.bucket}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Catalog Target Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = crawler::create(
///         "example",
///         CrawlerArgs::builder()
///             .catalog_targets(
///                 vec![
///                     CrawlerCatalogTarget::builder()
///                     .databaseName("${exampleAwsGlueCatalogDatabase.name}")
///                     .tables(vec!["${exampleAwsGlueCatalogTable.name}",]).build_struct(),
///                 ],
///             )
///             .configuration(
///                 "{\n  \"Version\":1.0,\n  \"Grouping\": {\n    \"TableGroupingPolicy\": \"CombineCompatibleSchemas\"\n  }\n}",
///             )
///             .database_name("${exampleAwsGlueCatalogDatabase.name}")
///             .name("example")
///             .role("${exampleAwsIamRole.arn}")
///             .schema_change_policy(
///                 CrawlerSchemaChangePolicy::builder().deleteBehavior("LOG").build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### MongoDB Target Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = crawler::create(
///         "example",
///         CrawlerArgs::builder()
///             .database_name("${exampleAwsGlueCatalogDatabase.name}")
///             .mongodb_targets(
///                 vec![
///                     CrawlerMongodbTarget::builder()
///                     .connectionName("${exampleAwsGlueConnection.name}")
///                     .path("database-name/%").build_struct(),
///                 ],
///             )
///             .name("example")
///             .role("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Configuration Settings Example
///
/// ```yaml
/// resources:
///   eventsCrawler:
///     type: aws:glue:Crawler
///     name: events_crawler
///     properties:
///       databaseName: ${glueDatabase.name}
///       schedule: cron(0 1 * * ? *)
///       name: events_crawler_${environmentName}
///       role: ${glueRole.arn}
///       tags: ${tags}
///       configuration:
///         fn::toJSON:
///           Grouping:
///             TableGroupingPolicy: CombineCompatibleSchemas
///           CrawlerOutput:
///             Partitions:
///               AddOrUpdateBehavior: InheritFromTable
///           Version: 1
///       s3Targets:
///         - path: s3://${dataLakeBucket.bucket}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Crawlers using `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/crawler:Crawler MyJob MyJob
/// ```
pub mod crawler {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CrawlerArgs {
        /// List of nested AWS Glue Data Catalog target arguments. See Catalog Target below.
        #[builder(into, default)]
        pub catalog_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerCatalogTarget>>,
        >,
        /// List of custom classifiers. By default, all AWS classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.
        #[builder(into, default)]
        pub classifiers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// JSON string of configuration information. For more details see [Setting Crawler Configuration Options](https://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html).
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Glue database where results are written.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// List of nested Delta Lake target arguments. See Delta Target below.
        #[builder(into, default)]
        pub delta_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerDeltaTarget>>,
        >,
        /// Description of the crawler.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// List of nested DynamoDB target arguments. See Dynamodb Target below.
        #[builder(into, default)]
        pub dynamodb_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerDynamodbTarget>>,
        >,
        /// List of nested Hudi target arguments. See Iceberg Target below.
        #[builder(into, default)]
        pub hudi_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerHudiTarget>>,
        >,
        /// List of nested Iceberg target arguments. See Iceberg Target below.
        #[builder(into, default)]
        pub iceberg_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerIcebergTarget>>,
        >,
        /// List of nested JDBC target arguments. See JDBC Target below.
        #[builder(into, default)]
        pub jdbc_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerJdbcTarget>>,
        >,
        /// Specifies Lake Formation configuration settings for the crawler. See Lake Formation Configuration below.
        #[builder(into, default)]
        pub lake_formation_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerLakeFormationConfiguration>,
        >,
        /// Specifies data lineage configuration settings for the crawler. See Lineage Configuration below.
        #[builder(into, default)]
        pub lineage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerLineageConfiguration>,
        >,
        /// List of nested MongoDB target arguments. See MongoDB Target below.
        #[builder(into, default)]
        pub mongodb_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerMongodbTarget>>,
        >,
        /// Name of the crawler.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.. See Recrawl Policy below.
        #[builder(into, default)]
        pub recrawl_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerRecrawlPolicy>,
        >,
        /// The IAM role friendly name (including path without leading slash), or ARN of an IAM role, used by the crawler to access other resources.
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
        /// List of nested Amazon S3 target arguments. See S3 Target below.
        #[builder(into, default)]
        pub s3_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerS3Target>>,
        >,
        /// A cron expression used to specify the schedule. For more information, see [Time-Based Schedules for Jobs and Crawlers](https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html). For example, to run something every day at 12:15 UTC, you would specify: `cron(15 12 * * ? *)`.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// Policy for the crawler's update and deletion behavior. See Schema Change Policy below.
        #[builder(into, default)]
        pub schema_change_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerSchemaChangePolicy>,
        >,
        /// The name of Security Configuration to be used by the crawler
        #[builder(into, default)]
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// The table prefix used for catalog tables that are created.
        #[builder(into, default)]
        pub table_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CrawlerResult {
        /// The ARN of the crawler
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of nested AWS Glue Data Catalog target arguments. See Catalog Target below.
        pub catalog_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerCatalogTarget>>,
        >,
        /// List of custom classifiers. By default, all AWS classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.
        pub classifiers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// JSON string of configuration information. For more details see [Setting Crawler Configuration Options](https://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html).
        pub configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// Glue database where results are written.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// List of nested Delta Lake target arguments. See Delta Target below.
        pub delta_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerDeltaTarget>>,
        >,
        /// Description of the crawler.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// List of nested DynamoDB target arguments. See Dynamodb Target below.
        pub dynamodb_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerDynamodbTarget>>,
        >,
        /// List of nested Hudi target arguments. See Iceberg Target below.
        pub hudi_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerHudiTarget>>,
        >,
        /// List of nested Iceberg target arguments. See Iceberg Target below.
        pub iceberg_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerIcebergTarget>>,
        >,
        /// List of nested JDBC target arguments. See JDBC Target below.
        pub jdbc_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerJdbcTarget>>,
        >,
        /// Specifies Lake Formation configuration settings for the crawler. See Lake Formation Configuration below.
        pub lake_formation_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerLakeFormationConfiguration>,
        >,
        /// Specifies data lineage configuration settings for the crawler. See Lineage Configuration below.
        pub lineage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerLineageConfiguration>,
        >,
        /// List of nested MongoDB target arguments. See MongoDB Target below.
        pub mongodb_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerMongodbTarget>>,
        >,
        /// Name of the crawler.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.. See Recrawl Policy below.
        pub recrawl_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerRecrawlPolicy>,
        >,
        /// The IAM role friendly name (including path without leading slash), or ARN of an IAM role, used by the crawler to access other resources.
        pub role: pulumi_wasm_rust::Output<String>,
        /// List of nested Amazon S3 target arguments. See S3 Target below.
        pub s3_targets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerS3Target>>,
        >,
        /// A cron expression used to specify the schedule. For more information, see [Time-Based Schedules for Jobs and Crawlers](https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html). For example, to run something every day at 12:15 UTC, you would specify: `cron(15 12 * * ? *)`.
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// Policy for the crawler's update and deletion behavior. See Schema Change Policy below.
        pub schema_change_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::CrawlerSchemaChangePolicy>,
        >,
        /// The name of Security Configuration to be used by the crawler
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// The table prefix used for catalog tables that are created.
        pub table_prefix: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn create(name: &str, args: CrawlerArgs) -> CrawlerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_targets_binding = args.catalog_targets.get_inner();
        let classifiers_binding = args.classifiers.get_inner();
        let configuration_binding = args.configuration.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let delta_targets_binding = args.delta_targets.get_inner();
        let description_binding = args.description.get_inner();
        let dynamodb_targets_binding = args.dynamodb_targets.get_inner();
        let hudi_targets_binding = args.hudi_targets.get_inner();
        let iceberg_targets_binding = args.iceberg_targets.get_inner();
        let jdbc_targets_binding = args.jdbc_targets.get_inner();
        let lake_formation_configuration_binding = args
            .lake_formation_configuration
            .get_inner();
        let lineage_configuration_binding = args.lineage_configuration.get_inner();
        let mongodb_targets_binding = args.mongodb_targets.get_inner();
        let name_binding = args.name.get_inner();
        let recrawl_policy_binding = args.recrawl_policy.get_inner();
        let role_binding = args.role.get_inner();
        let s3_targets_binding = args.s3_targets.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let schema_change_policy_binding = args.schema_change_policy.get_inner();
        let security_configuration_binding = args.security_configuration.get_inner();
        let table_prefix_binding = args.table_prefix.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/crawler:Crawler".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogTargets".into(),
                    value: &catalog_targets_binding,
                },
                register_interface::ObjectField {
                    name: "classifiers".into(),
                    value: &classifiers_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "deltaTargets".into(),
                    value: &delta_targets_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dynamodbTargets".into(),
                    value: &dynamodb_targets_binding,
                },
                register_interface::ObjectField {
                    name: "hudiTargets".into(),
                    value: &hudi_targets_binding,
                },
                register_interface::ObjectField {
                    name: "icebergTargets".into(),
                    value: &iceberg_targets_binding,
                },
                register_interface::ObjectField {
                    name: "jdbcTargets".into(),
                    value: &jdbc_targets_binding,
                },
                register_interface::ObjectField {
                    name: "lakeFormationConfiguration".into(),
                    value: &lake_formation_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "lineageConfiguration".into(),
                    value: &lineage_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "mongodbTargets".into(),
                    value: &mongodb_targets_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recrawlPolicy".into(),
                    value: &recrawl_policy_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "s3Targets".into(),
                    value: &s3_targets_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "schemaChangePolicy".into(),
                    value: &schema_change_policy_binding,
                },
                register_interface::ObjectField {
                    name: "securityConfiguration".into(),
                    value: &security_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tablePrefix".into(),
                    value: &table_prefix_binding,
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
                    name: "catalogTargets".into(),
                },
                register_interface::ResultField {
                    name: "classifiers".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "deltaTargets".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dynamodbTargets".into(),
                },
                register_interface::ResultField {
                    name: "hudiTargets".into(),
                },
                register_interface::ResultField {
                    name: "icebergTargets".into(),
                },
                register_interface::ResultField {
                    name: "jdbcTargets".into(),
                },
                register_interface::ResultField {
                    name: "lakeFormationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "lineageConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "mongodbTargets".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recrawlPolicy".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
                register_interface::ResultField {
                    name: "s3Targets".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "schemaChangePolicy".into(),
                },
                register_interface::ResultField {
                    name: "securityConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tablePrefix".into(),
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
        CrawlerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            catalog_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogTargets").unwrap(),
            ),
            classifiers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("classifiers").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            delta_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deltaTargets").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dynamodb_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dynamodbTargets").unwrap(),
            ),
            hudi_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hudiTargets").unwrap(),
            ),
            iceberg_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("icebergTargets").unwrap(),
            ),
            jdbc_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jdbcTargets").unwrap(),
            ),
            lake_formation_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lakeFormationConfiguration").unwrap(),
            ),
            lineage_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lineageConfiguration").unwrap(),
            ),
            mongodb_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mongodbTargets").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recrawl_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recrawlPolicy").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
            s3_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Targets").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            schema_change_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaChangePolicy").unwrap(),
            ),
            security_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityConfiguration").unwrap(),
            ),
            table_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tablePrefix").unwrap(),
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