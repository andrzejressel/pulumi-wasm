/// Resource for managing an AWS Kendra Data Source.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:DataSource
///     properties:
///       indexId: ${exampleAwsKendraIndex.id}
///       name: example
///       description: example
///       languageCode: en
///       type: CUSTOM
///       tags:
///         hello: world
/// ```
///
/// ### S3 Connector
///
/// ### With Schedule
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .s3Configuration(
///                         DataSourceConfigurationS3Configuration::builder()
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .schedule("cron(9 10 1 * ? *)")
///             .type_("S3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Access Control List
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .s3Configuration(
///                         DataSourceConfigurationS3Configuration::builder()
///                             .accessControlListConfiguration(
///                                 DataSourceConfigurationS3ConfigurationAccessControlListConfiguration::builder()
///                                     .keyPath("s3://${exampleAwsS3Bucket.id}/path-1")
///                                     .build_struct(),
///                             )
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("S3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Documents Metadata Configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .s3Configuration(
///                         DataSourceConfigurationS3Configuration::builder()
///                             .bucketName("${exampleAwsS3Bucket.id}")
///                             .documentsMetadataConfiguration(
///                                 DataSourceConfigurationS3ConfigurationDocumentsMetadataConfiguration::builder()
///                                     .s3Prefix("example")
///                                     .build_struct(),
///                             )
///                             .exclusionPatterns(vec!["example",])
///                             .inclusionPatterns(vec!["hello",])
///                             .inclusionPrefixes(vec!["world",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("S3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Web Crawler Connector
///
/// ### With Seed URLs
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .seedUrlConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration::builder()
///                                             .seedUrls(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Site Maps
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .siteMapsConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSiteMapsConfiguration::builder()
///                                             .siteMaps(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Web Crawler Mode
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .seedUrlConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration::builder()
///                                             .seedUrls(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .webCrawlerMode("SUBDOMAINS")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Authentication Configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:DataSource
///     properties:
///       indexId: ${exampleAwsKendraIndex.id}
///       name: example
///       type: WEBCRAWLER
///       roleArn: ${exampleAwsIamRole.arn}
///       configuration:
///         webCrawlerConfiguration:
///           authenticationConfiguration:
///             basicAuthentications:
///               - credentials: ${exampleAwsSecretsmanagerSecret.arn}
///                 host: a.example.com
///                 port: '443'
///           urls:
///             seedUrlConfiguration:
///               seedUrls:
///                 - REPLACE_WITH_YOUR_URL
///     options:
///       dependsOn:
///         - ${exampleAwsSecretsmanagerSecretVersion}
/// ```
///
/// ### With Crawl Depth
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .crawlDepth(3)
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .seedUrlConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration::builder()
///                                             .seedUrls(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Max Links Per Page
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .maxLinksPerPage(100)
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .seedUrlConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration::builder()
///                                             .seedUrls(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Max Urls Per Minute Crawl Rate
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .maxUrlsPerMinuteCrawlRate(300)
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .seedUrlConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration::builder()
///                                             .seedUrls(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Proxy Configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:DataSource
///     properties:
///       indexId: ${exampleAwsKendraIndex.id}
///       name: example
///       type: WEBCRAWLER
///       roleArn: ${exampleAwsIamRole.arn}
///       configuration:
///         webCrawlerConfiguration:
///           proxyConfiguration:
///             credentials: ${exampleAwsSecretsmanagerSecret.arn}
///             host: a.example.com
///             port: '443'
///           urls:
///             seedUrlConfiguration:
///               seedUrls:
///                 - REPLACE_WITH_YOUR_URL
///     options:
///       dependsOn:
///         - ${exampleAwsSecretsmanagerSecretVersion}
/// ```
///
/// ### With URL Exclusion and Inclusion Patterns
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_source::create(
///         "example",
///         DataSourceArgs::builder()
///             .configuration(
///                 DataSourceConfiguration::builder()
///                     .webCrawlerConfiguration(
///                         DataSourceConfigurationWebCrawlerConfiguration::builder()
///                             .urlExclusionPatterns(vec!["example",])
///                             .urlInclusionPatterns(vec!["hello",])
///                             .urls(
///                                 DataSourceConfigurationWebCrawlerConfigurationUrls::builder()
///                                     .seedUrlConfiguration(
///                                         DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration::builder()
///                                             .seedUrls(vec!["REPLACE_WITH_YOUR_URL",])
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .type_("WEBCRAWLER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kendra Data Source using the unique identifiers of the data_source and index separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:kendra/dataSource:DataSource example 1045d08d-66ef-4882-b3ed-dfb7df183e90/b34dfdf7-1f2b-4704-9581-79e00296845f
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceArgs {
        /// A block with the configuration information to connect to your Data Source repository. You can't specify the `configuration` block when the `type` parameter is set to `CUSTOM`. Detailed below.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kendra::DataSourceConfiguration>,
        >,
        /// A block with the configuration information for altering document metadata and content during the document ingestion process. For more information on how to create, modify and delete document metadata, or make other content alterations when you ingest documents into Amazon Kendra, see [Customizing document metadata during the ingestion process](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html). Detailed below.
        #[builder(into, default)]
        pub custom_document_enrichment_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfiguration,
            >,
        >,
        /// A description for the Data Source connector.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the index for your Amazon Kendra data source.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The code for a language. This allows you to support a language for all documents when creating the Data Source connector. English is supported by default. For more information on supported languages, including their codes, see [Adding documents in languages other than English](https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html).
        #[builder(into, default)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name for your data source connector.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the data source connector. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html). You can't specify the `role_arn` parameter when the `type` parameter is set to `CUSTOM`. The `role_arn` parameter is required for all other data sources.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Sets the frequency for Amazon Kendra to check the documents in your Data Source repository and update the index. If you don't set a schedule Amazon Kendra will not periodically update the index. You can call the `StartDataSourceSyncJob` API to update the index.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of data source repository. For an updated list of values, refer to [Valid Values for Type](https://docs.aws.amazon.com/kendra/latest/dg/API_CreateDataSource.html#Kendra-CreateDataSource-request-Type).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceResult {
        /// ARN of the Data Source.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A block with the configuration information to connect to your Data Source repository. You can't specify the `configuration` block when the `type` parameter is set to `CUSTOM`. Detailed below.
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::kendra::DataSourceConfiguration>,
        >,
        /// The Unix timestamp of when the Data Source was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// A block with the configuration information for altering document metadata and content during the document ingestion process. For more information on how to create, modify and delete document metadata, or make other content alterations when you ingest documents into Amazon Kendra, see [Customizing document metadata during the ingestion process](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html). Detailed below.
        pub custom_document_enrichment_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfiguration,
            >,
        >,
        /// The unique identifiers of the Data Source.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// A description for the Data Source connector.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// When the Status field value is `FAILED`, the ErrorMessage field contains a description of the error that caused the Data Source to fail.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the index for your Amazon Kendra data source.
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// The code for a language. This allows you to support a language for all documents when creating the Data Source connector. English is supported by default. For more information on supported languages, including their codes, see [Adding documents in languages other than English](https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html).
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// A name for your data source connector.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the data source connector. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html). You can't specify the `role_arn` parameter when the `type` parameter is set to `CUSTOM`. The `role_arn` parameter is required for all other data sources.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Sets the frequency for Amazon Kendra to check the documents in your Data Source repository and update the index. If you don't set a schedule Amazon Kendra will not periodically update the index. You can call the `StartDataSourceSyncJob` API to update the index.
        pub schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// The current status of the Data Source. When the status is `ACTIVE` the Data Source is ready to use. When the status is `FAILED`, the `error_message` field contains the reason that the Data Source failed.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of data source repository. For an updated list of values, refer to [Valid Values for Type](https://docs.aws.amazon.com/kendra/latest/dg/API_CreateDataSource.html#Kendra-CreateDataSource-request-Type).
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The Unix timestamp of when the Data Source was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataSourceArgs,
    ) -> DataSourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let custom_document_enrichment_configuration_binding = args
            .custom_document_enrichment_configuration
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let index_id_binding = args.index_id.get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kendra/dataSource:DataSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDocumentEnrichmentConfiguration".into(),
                    value: custom_document_enrichment_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexId".into(),
                    value: index_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataSourceResult {
            arn: o.get_field("arn"),
            configuration: o.get_field("configuration"),
            created_at: o.get_field("createdAt"),
            custom_document_enrichment_configuration: o
                .get_field("customDocumentEnrichmentConfiguration"),
            data_source_id: o.get_field("dataSourceId"),
            description: o.get_field("description"),
            error_message: o.get_field("errorMessage"),
            index_id: o.get_field("indexId"),
            language_code: o.get_field("languageCode"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            schedule: o.get_field("schedule"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
