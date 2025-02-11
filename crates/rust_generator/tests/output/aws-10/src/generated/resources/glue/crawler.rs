/// Manages a Glue Crawler. More information can be found in the [AWS Glue Developer Guide](https://docs.aws.amazon.com/glue/latest/dg/add-crawler.html)
///
/// ## Example Usage
///
/// ### DynamoDB Target Example
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod crawler {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CrawlerArgs {
        /// List of nested AWS Glue Data Catalog target arguments. See Catalog Target below.
        #[builder(into, default)]
        pub catalog_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerCatalogTarget>>,
        >,
        /// List of custom classifiers. By default, all AWS classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.
        #[builder(into, default)]
        pub classifiers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// JSON string of configuration information. For more details see [Setting Crawler Configuration Options](https://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html).
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Glue database where results are written.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of nested Delta Lake target arguments. See Delta Target below.
        #[builder(into, default)]
        pub delta_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerDeltaTarget>>,
        >,
        /// Description of the crawler.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of nested DynamoDB target arguments. See Dynamodb Target below.
        #[builder(into, default)]
        pub dynamodb_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerDynamodbTarget>>,
        >,
        /// List of nested Hudi target arguments. See Iceberg Target below.
        #[builder(into, default)]
        pub hudi_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerHudiTarget>>,
        >,
        /// List of nested Iceberg target arguments. See Iceberg Target below.
        #[builder(into, default)]
        pub iceberg_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerIcebergTarget>>,
        >,
        /// List of nested JDBC target arguments. See JDBC Target below.
        #[builder(into, default)]
        pub jdbc_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerJdbcTarget>>,
        >,
        /// Specifies Lake Formation configuration settings for the crawler. See Lake Formation Configuration below.
        #[builder(into, default)]
        pub lake_formation_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CrawlerLakeFormationConfiguration>,
        >,
        /// Specifies data lineage configuration settings for the crawler. See Lineage Configuration below.
        #[builder(into, default)]
        pub lineage_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CrawlerLineageConfiguration>,
        >,
        /// List of nested MongoDB target arguments. See MongoDB Target below.
        #[builder(into, default)]
        pub mongodb_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerMongodbTarget>>,
        >,
        /// Name of the crawler.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.. See Recrawl Policy below.
        #[builder(into, default)]
        pub recrawl_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CrawlerRecrawlPolicy>,
        >,
        /// The IAM role friendly name (including path without leading slash), or ARN of an IAM role, used by the crawler to access other resources.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of nested Amazon S3 target arguments. See S3 Target below.
        #[builder(into, default)]
        pub s3_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::CrawlerS3Target>>,
        >,
        /// A cron expression used to specify the schedule. For more information, see [Time-Based Schedules for Jobs and Crawlers](https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html). For example, to run something every day at 12:15 UTC, you would specify: `cron(15 12 * * ? *)`.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy for the crawler's update and deletion behavior. See Schema Change Policy below.
        #[builder(into, default)]
        pub schema_change_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::CrawlerSchemaChangePolicy>,
        >,
        /// The name of Security Configuration to be used by the crawler
        #[builder(into, default)]
        pub security_configuration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The table prefix used for catalog tables that are created.
        #[builder(into, default)]
        pub table_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CrawlerResult {
        /// The ARN of the crawler
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of nested AWS Glue Data Catalog target arguments. See Catalog Target below.
        pub catalog_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerCatalogTarget>>,
        >,
        /// List of custom classifiers. By default, all AWS classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.
        pub classifiers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// JSON string of configuration information. For more details see [Setting Crawler Configuration Options](https://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html).
        pub configuration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Glue database where results are written.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// List of nested Delta Lake target arguments. See Delta Target below.
        pub delta_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerDeltaTarget>>,
        >,
        /// Description of the crawler.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of nested DynamoDB target arguments. See Dynamodb Target below.
        pub dynamodb_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerDynamodbTarget>>,
        >,
        /// List of nested Hudi target arguments. See Iceberg Target below.
        pub hudi_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerHudiTarget>>,
        >,
        /// List of nested Iceberg target arguments. See Iceberg Target below.
        pub iceberg_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerIcebergTarget>>,
        >,
        /// List of nested JDBC target arguments. See JDBC Target below.
        pub jdbc_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerJdbcTarget>>,
        >,
        /// Specifies Lake Formation configuration settings for the crawler. See Lake Formation Configuration below.
        pub lake_formation_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CrawlerLakeFormationConfiguration>,
        >,
        /// Specifies data lineage configuration settings for the crawler. See Lineage Configuration below.
        pub lineage_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CrawlerLineageConfiguration>,
        >,
        /// List of nested MongoDB target arguments. See MongoDB Target below.
        pub mongodb_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerMongodbTarget>>,
        >,
        /// Name of the crawler.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.. See Recrawl Policy below.
        pub recrawl_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CrawlerRecrawlPolicy>,
        >,
        /// The IAM role friendly name (including path without leading slash), or ARN of an IAM role, used by the crawler to access other resources.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// List of nested Amazon S3 target arguments. See S3 Target below.
        pub s3_targets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::CrawlerS3Target>>,
        >,
        /// A cron expression used to specify the schedule. For more information, see [Time-Based Schedules for Jobs and Crawlers](https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html). For example, to run something every day at 12:15 UTC, you would specify: `cron(15 12 * * ? *)`.
        pub schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// Policy for the crawler's update and deletion behavior. See Schema Change Policy below.
        pub schema_change_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CrawlerSchemaChangePolicy>,
        >,
        /// The name of Security Configuration to be used by the crawler
        pub security_configuration: pulumi_gestalt_rust::Output<Option<String>>,
        /// The table prefix used for catalog tables that are created.
        pub table_prefix: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: CrawlerArgs,
    ) -> CrawlerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_targets_binding = args.catalog_targets.get_output(context);
        let classifiers_binding = args.classifiers.get_output(context);
        let configuration_binding = args.configuration.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let delta_targets_binding = args.delta_targets.get_output(context);
        let description_binding = args.description.get_output(context);
        let dynamodb_targets_binding = args.dynamodb_targets.get_output(context);
        let hudi_targets_binding = args.hudi_targets.get_output(context);
        let iceberg_targets_binding = args.iceberg_targets.get_output(context);
        let jdbc_targets_binding = args.jdbc_targets.get_output(context);
        let lake_formation_configuration_binding = args
            .lake_formation_configuration
            .get_output(context);
        let lineage_configuration_binding = args
            .lineage_configuration
            .get_output(context);
        let mongodb_targets_binding = args.mongodb_targets.get_output(context);
        let name_binding = args.name.get_output(context);
        let recrawl_policy_binding = args.recrawl_policy.get_output(context);
        let role_binding = args.role.get_output(context);
        let s3_targets_binding = args.s3_targets.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let schema_change_policy_binding = args.schema_change_policy.get_output(context);
        let security_configuration_binding = args
            .security_configuration
            .get_output(context);
        let table_prefix_binding = args.table_prefix.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/crawler:Crawler".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogTargets".into(),
                    value: &catalog_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "classifiers".into(),
                    value: &classifiers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deltaTargets".into(),
                    value: &delta_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamodbTargets".into(),
                    value: &dynamodb_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hudiTargets".into(),
                    value: &hudi_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "icebergTargets".into(),
                    value: &iceberg_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jdbcTargets".into(),
                    value: &jdbc_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lakeFormationConfiguration".into(),
                    value: &lake_formation_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lineageConfiguration".into(),
                    value: &lineage_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mongodbTargets".into(),
                    value: &mongodb_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recrawlPolicy".into(),
                    value: &recrawl_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Targets".into(),
                    value: &s3_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaChangePolicy".into(),
                    value: &schema_change_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityConfiguration".into(),
                    value: &security_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tablePrefix".into(),
                    value: &table_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CrawlerResult {
            arn: o.get_field("arn"),
            catalog_targets: o.get_field("catalogTargets"),
            classifiers: o.get_field("classifiers"),
            configuration: o.get_field("configuration"),
            database_name: o.get_field("databaseName"),
            delta_targets: o.get_field("deltaTargets"),
            description: o.get_field("description"),
            dynamodb_targets: o.get_field("dynamodbTargets"),
            hudi_targets: o.get_field("hudiTargets"),
            iceberg_targets: o.get_field("icebergTargets"),
            jdbc_targets: o.get_field("jdbcTargets"),
            lake_formation_configuration: o.get_field("lakeFormationConfiguration"),
            lineage_configuration: o.get_field("lineageConfiguration"),
            mongodb_targets: o.get_field("mongodbTargets"),
            name: o.get_field("name"),
            recrawl_policy: o.get_field("recrawlPolicy"),
            role: o.get_field("role"),
            s3_targets: o.get_field("s3Targets"),
            schedule: o.get_field("schedule"),
            schema_change_policy: o.get_field("schemaChangePolicy"),
            security_configuration: o.get_field("securityConfiguration"),
            table_prefix: o.get_field("tablePrefix"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
