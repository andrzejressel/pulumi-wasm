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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod data_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceArgs {
        /// A block with the configuration information to connect to your Data Source repository. You can't specify the `configuration` block when the `type` parameter is set to `CUSTOM`. Detailed below.
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::kendra::DataSourceConfiguration>,
        >,
        /// A block with the configuration information for altering document metadata and content during the document ingestion process. For more information on how to create, modify and delete document metadata, or make other content alterations when you ingest documents into Amazon Kendra, see [Customizing document metadata during the ingestion process](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html). Detailed below.
        #[builder(into, default)]
        pub custom_document_enrichment_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfiguration,
            >,
        >,
        /// A description for the Data Source connector.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the index for your Amazon Kendra data source.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// The code for a language. This allows you to support a language for all documents when creating the Data Source connector. English is supported by default. For more information on supported languages, including their codes, see [Adding documents in languages other than English](https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html).
        #[builder(into, default)]
        pub language_code: pulumi_wasm_rust::Output<Option<String>>,
        /// A name for your data source connector.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the data source connector. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html). You can't specify the `role_arn` parameter when the `type` parameter is set to `CUSTOM`. The `role_arn` parameter is required for all other data sources.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the frequency for Amazon Kendra to check the documents in your Data Source repository and update the index. If you don't set a schedule Amazon Kendra will not periodically update the index. You can call the `StartDataSourceSyncJob` API to update the index.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of data source repository. For an updated list of values, refer to [Valid Values for Type](https://docs.aws.amazon.com/kendra/latest/dg/API_CreateDataSource.html#Kendra-CreateDataSource-request-Type).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceResult {
        /// ARN of the Data Source.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A block with the configuration information to connect to your Data Source repository. You can't specify the `configuration` block when the `type` parameter is set to `CUSTOM`. Detailed below.
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::kendra::DataSourceConfiguration>,
        >,
        /// The Unix timestamp of when the Data Source was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A block with the configuration information for altering document metadata and content during the document ingestion process. For more information on how to create, modify and delete document metadata, or make other content alterations when you ingest documents into Amazon Kendra, see [Customizing document metadata during the ingestion process](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html). Detailed below.
        pub custom_document_enrichment_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfiguration,
            >,
        >,
        /// The unique identifiers of the Data Source.
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// A description for the Data Source connector.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// When the Status field value is `FAILED`, the ErrorMessage field contains a description of the error that caused the Data Source to fail.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// The identifier of the index for your Amazon Kendra data source.
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// The code for a language. This allows you to support a language for all documents when creating the Data Source connector. English is supported by default. For more information on supported languages, including their codes, see [Adding documents in languages other than English](https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html).
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// A name for your data source connector.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the data source connector. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html). You can't specify the `role_arn` parameter when the `type` parameter is set to `CUSTOM`. The `role_arn` parameter is required for all other data sources.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the frequency for Amazon Kendra to check the documents in your Data Source repository and update the index. If you don't set a schedule Amazon Kendra will not periodically update the index. You can call the `StartDataSourceSyncJob` API to update the index.
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// The current status of the Data Source. When the status is `ACTIVE` the Data Source is ready to use. When the status is `FAILED`, the `error_message` field contains the reason that the Data Source failed.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of data source repository. For an updated list of values, refer to [Valid Values for Type](https://docs.aws.amazon.com/kendra/latest/dg/API_CreateDataSource.html#Kendra-CreateDataSource-request-Type).
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The Unix timestamp of when the Data Source was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataSourceArgs) -> DataSourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let custom_document_enrichment_configuration_binding = args
            .custom_document_enrichment_configuration
            .get_inner();
        let description_binding = args.description.get_inner();
        let index_id_binding = args.index_id.get_inner();
        let language_code_binding = args.language_code.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kendra/dataSource:DataSource".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "customDocumentEnrichmentConfiguration".into(),
                    value: &custom_document_enrichment_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "customDocumentEnrichmentConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "errorMessage".into(),
                },
                register_interface::ResultField {
                    name: "indexId".into(),
                },
                register_interface::ResultField {
                    name: "languageCode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataSourceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            custom_document_enrichment_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customDocumentEnrichmentConfiguration").unwrap(),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            error_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorMessage").unwrap(),
            ),
            index_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexId").unwrap(),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageCode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}
