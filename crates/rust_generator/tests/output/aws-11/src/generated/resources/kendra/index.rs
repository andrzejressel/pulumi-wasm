/// Provides an Amazon Kendra Index resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:Index
///     properties:
///       name: example
///       description: example
///       edition: DEVELOPER_EDITION
///       roleArn: ${this.arn}
///       tags:
///         Key1: Value1
/// ```
///
/// ### With capacity units
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = index::create(
///         "example",
///         IndexArgs::builder()
///             .capacity_units(
///                 IndexCapacityUnits::builder()
///                     .queryCapacityUnits(2)
///                     .storageCapacityUnits(2)
///                     .build_struct(),
///             )
///             .edition("DEVELOPER_EDITION")
///             .name("example")
///             .role_arn("${this.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With server side encryption configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = index::create(
///         "example",
///         IndexArgs::builder()
///             .name("example")
///             .role_arn("${thisAwsIamRole.arn}")
///             .server_side_encryption_configuration(
///                 IndexServerSideEncryptionConfiguration::builder()
///                     .kmsKeyId("${this.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With user group resolution configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = index::create(
///         "example",
///         IndexArgs::builder()
///             .name("example")
///             .role_arn("${this.arn}")
///             .user_group_resolution_configuration(
///                 IndexUserGroupResolutionConfiguration::builder()
///                     .userGroupResolutionMode("AWS_SSO")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Document Metadata Configuration Updates
///
/// ### Specifying the predefined elements
///
/// Refer to [Amazon Kendra documentation on built-in document fields](https://docs.aws.amazon.com/kendra/latest/dg/hiw-index.html#index-reserved-fields) for more information.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:Index
///     properties:
///       name: example
///       roleArn: ${this.arn}
///       documentMetadataConfigurationUpdates:
///         - name: _authors
///           type: STRING_LIST_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: false
///           relevance:
///             importance: 1
///         - name: _category
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _created_at
///           type: DATE_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             freshness: false
///             importance: 1
///             duration: 25920000s
///             rankOrder: ASCENDING
///         - name: _data_source_id
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _document_title
///           type: STRING_VALUE
///           search:
///             displayable: true
///             facetable: false
///             searchable: true
///             sortable: true
///           relevance:
///             importance: 2
///             valuesImportanceMap: {}
///         - name: _excerpt_page_number
///           type: LONG_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: false
///           relevance:
///             importance: 2
///             rankOrder: ASCENDING
///         - name: _faq_id
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _file_type
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _language_code
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _last_updated_at
///           type: DATE_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             freshness: false
///             importance: 1
///             duration: 25920000s
///             rankOrder: ASCENDING
///         - name: _source_uri
///           type: STRING_VALUE
///           search:
///             displayable: true
///             facetable: false
///             searchable: false
///             sortable: false
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _tenant_id
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _version
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _view_count
///           type: LONG_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             rankOrder: ASCENDING
/// ```
///
/// ### Appending additional elements
///
/// The example below shows additional elements with names, `example-string-value`, `example-long-value`, `example-string-list-value`, `example-date-value` representing the 4 types of `STRING_VALUE`, `LONG_VALUE`, `STRING_LIST_VALUE`, `DATE_VALUE` respectively.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:Index
///     properties:
///       name: example
///       roleArn: ${this.arn}
///       documentMetadataConfigurationUpdates:
///         - name: _authors
///           type: STRING_LIST_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: false
///           relevance:
///             importance: 1
///         - name: _category
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _created_at
///           type: DATE_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             freshness: false
///             importance: 1
///             duration: 25920000s
///             rankOrder: ASCENDING
///         - name: _data_source_id
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _document_title
///           type: STRING_VALUE
///           search:
///             displayable: true
///             facetable: false
///             searchable: true
///             sortable: true
///           relevance:
///             importance: 2
///             valuesImportanceMap: {}
///         - name: _excerpt_page_number
///           type: LONG_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: false
///           relevance:
///             importance: 2
///             rankOrder: ASCENDING
///         - name: _faq_id
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _file_type
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _language_code
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _last_updated_at
///           type: DATE_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             freshness: false
///             importance: 1
///             duration: 25920000s
///             rankOrder: ASCENDING
///         - name: _source_uri
///           type: STRING_VALUE
///           search:
///             displayable: true
///             facetable: false
///             searchable: false
///             sortable: false
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _tenant_id
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _version
///           type: STRING_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: _view_count
///           type: LONG_VALUE
///           search:
///             displayable: false
///             facetable: false
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             rankOrder: ASCENDING
///         - name: example-string-value
///           type: STRING_VALUE
///           search:
///             displayable: true
///             facetable: true
///             searchable: true
///             sortable: true
///           relevance:
///             importance: 1
///             valuesImportanceMap: {}
///         - name: example-long-value
///           type: LONG_VALUE
///           search:
///             displayable: true
///             facetable: true
///             searchable: false
///             sortable: true
///           relevance:
///             importance: 1
///             rankOrder: ASCENDING
///         - name: example-string-list-value
///           type: STRING_LIST_VALUE
///           search:
///             displayable: true
///             facetable: true
///             searchable: true
///             sortable: false
///           relevance:
///             importance: 1
///         - name: example-date-value
///           type: DATE_VALUE
///           search:
///             displayable: true
///             facetable: true
///             searchable: false
///             sortable: false
///           relevance:
///             freshness: false
///             importance: 1
///             duration: 25920000s
///             rankOrder: ASCENDING
/// ```
///
/// ### With JSON token type configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = index::create(
///         "example",
///         IndexArgs::builder()
///             .name("example")
///             .role_arn("${this.arn}")
///             .user_token_configurations(
///                 IndexUserTokenConfigurations::builder()
///                     .jsonTokenTypeConfiguration(
///                         IndexUserTokenConfigurationsJsonTokenTypeConfiguration::builder()
///                             .groupAttributeField("groups")
///                             .userNameAttributeField("username")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Kendra Indexes using its `id`. For example:
///
/// ```sh
/// $ pulumi import aws:kendra/index:Index example 12345678-1234-5678-9123-123456789123
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IndexArgs {
        /// A block that sets the number of additional document storage and query capacity units that should be used by the index. Detailed below.
        #[builder(into, default)]
        pub capacity_units: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kendra::IndexCapacityUnits>,
        >,
        /// The description of the Index.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more blocks that specify the configuration settings for any metadata applied to the documents in the index. Minimum number of 0 items. Maximum number of 500 items. If specified, you must define all elements, including those that are provided by default. These index fields are documented at [Amazon Kendra Index documentation](https://docs.aws.amazon.com/kendra/latest/dg/hiw-index.html). For an example resource that defines these default index fields, refer to the default example above. For an example resource that appends additional index fields, refer to the append example above. All arguments for each block must be specified. Note that blocks cannot be removed since index fields cannot be deleted. This argument is detailed below.
        #[builder(into, default)]
        pub document_metadata_configuration_updates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::kendra::IndexDocumentMetadataConfigurationUpdate,
                >,
            >,
        >,
        /// The Amazon Kendra edition to use for the index. Choose `DEVELOPER_EDITION` for indexes intended for development, testing, or proof of concept. Use `ENTERPRISE_EDITION` for your production databases. Once you set the edition for an index, it can't be changed. Defaults to `ENTERPRISE_EDITION`
        #[builder(into, default)]
        pub edition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Index.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access your Amazon CloudWatch logs and metrics. This is also the role you use when you call the `BatchPutDocument` API to index documents from an Amazon S3 bucket.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A block that specifies the identifier of the AWS KMS customer managed key (CMK) that's used to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs. Detailed below.
        #[builder(into, default)]
        pub server_side_encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kendra::IndexServerSideEncryptionConfiguration>,
        >,
        /// Tags to apply to the Index. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user context policy. Valid values are `ATTRIBUTE_FILTER` or `USER_TOKEN`. For more information, refer to [UserContextPolicy](https://docs.aws.amazon.com/kendra/latest/APIReference/API_CreateIndex.html#kendra-CreateIndex-request-UserContextPolicy). Defaults to `ATTRIBUTE_FILTER`.
        #[builder(into, default)]
        pub user_context_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block that enables fetching access levels of groups and users from an AWS Single Sign-On identity source. To configure this, see [UserGroupResolutionConfiguration](https://docs.aws.amazon.com/kendra/latest/dg/API_UserGroupResolutionConfiguration.html). Detailed below.
        #[builder(into, default)]
        pub user_group_resolution_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kendra::IndexUserGroupResolutionConfiguration>,
        >,
        /// A block that specifies the user token configuration. Detailed below.
        #[builder(into, default)]
        pub user_token_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kendra::IndexUserTokenConfigurations>,
        >,
    }
    #[allow(dead_code)]
    pub struct IndexResult {
        /// The Amazon Resource Name (ARN) of the Index.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A block that sets the number of additional document storage and query capacity units that should be used by the index. Detailed below.
        pub capacity_units: pulumi_gestalt_rust::Output<
            super::super::types::kendra::IndexCapacityUnits,
        >,
        /// The Unix datetime that the index was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The description of the Index.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more blocks that specify the configuration settings for any metadata applied to the documents in the index. Minimum number of 0 items. Maximum number of 500 items. If specified, you must define all elements, including those that are provided by default. These index fields are documented at [Amazon Kendra Index documentation](https://docs.aws.amazon.com/kendra/latest/dg/hiw-index.html). For an example resource that defines these default index fields, refer to the default example above. For an example resource that appends additional index fields, refer to the append example above. All arguments for each block must be specified. Note that blocks cannot be removed since index fields cannot be deleted. This argument is detailed below.
        pub document_metadata_configuration_updates: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kendra::IndexDocumentMetadataConfigurationUpdate>,
        >,
        /// The Amazon Kendra edition to use for the index. Choose `DEVELOPER_EDITION` for indexes intended for development, testing, or proof of concept. Use `ENTERPRISE_EDITION` for your production databases. Once you set the edition for an index, it can't be changed. Defaults to `ENTERPRISE_EDITION`
        pub edition: pulumi_gestalt_rust::Output<Option<String>>,
        /// When the Status field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// A block that provides information about the number of FAQ questions and answers and the number of text documents indexed. Detailed below.
        pub index_statistics: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kendra::IndexIndexStatistic>,
        >,
        /// Specifies the name of the Index.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access your Amazon CloudWatch logs and metrics. This is also the role you use when you call the `BatchPutDocument` API to index documents from an Amazon S3 bucket.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A block that specifies the identifier of the AWS KMS customer managed key (CMK) that's used to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs. Detailed below.
        pub server_side_encryption_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::kendra::IndexServerSideEncryptionConfiguration>,
        >,
        /// The current status of the index. When the value is `ACTIVE`, the index is ready for use. If the Status field value is `FAILED`, the `error_message` field contains a message that explains why.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Index. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Unix datetime that the index was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
        /// The user context policy. Valid values are `ATTRIBUTE_FILTER` or `USER_TOKEN`. For more information, refer to [UserContextPolicy](https://docs.aws.amazon.com/kendra/latest/APIReference/API_CreateIndex.html#kendra-CreateIndex-request-UserContextPolicy). Defaults to `ATTRIBUTE_FILTER`.
        pub user_context_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block that enables fetching access levels of groups and users from an AWS Single Sign-On identity source. To configure this, see [UserGroupResolutionConfiguration](https://docs.aws.amazon.com/kendra/latest/dg/API_UserGroupResolutionConfiguration.html). Detailed below.
        pub user_group_resolution_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::kendra::IndexUserGroupResolutionConfiguration>,
        >,
        /// A block that specifies the user token configuration. Detailed below.
        pub user_token_configurations: pulumi_gestalt_rust::Output<
            Option<super::super::types::kendra::IndexUserTokenConfigurations>,
        >,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_units_binding = args.capacity_units.get_output(context);
        let description_binding = args.description.get_output(context);
        let document_metadata_configuration_updates_binding = args
            .document_metadata_configuration_updates
            .get_output(context);
        let edition_binding = args.edition.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let server_side_encryption_configuration_binding = args
            .server_side_encryption_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_context_policy_binding = args.user_context_policy.get_output(context);
        let user_group_resolution_configuration_binding = args
            .user_group_resolution_configuration
            .get_output(context);
        let user_token_configurations_binding = args
            .user_token_configurations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kendra/index:Index".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityUnits".into(),
                    value: capacity_units_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentMetadataConfigurationUpdates".into(),
                    value: document_metadata_configuration_updates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edition".into(),
                    value: edition_binding.get_id(),
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
                    name: "serverSideEncryptionConfiguration".into(),
                    value: server_side_encryption_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userContextPolicy".into(),
                    value: user_context_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userGroupResolutionConfiguration".into(),
                    value: user_group_resolution_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userTokenConfigurations".into(),
                    value: user_token_configurations_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IndexResult {
            arn: o.get_field("arn"),
            capacity_units: o.get_field("capacityUnits"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            document_metadata_configuration_updates: o
                .get_field("documentMetadataConfigurationUpdates"),
            edition: o.get_field("edition"),
            error_message: o.get_field("errorMessage"),
            index_statistics: o.get_field("indexStatistics"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            server_side_encryption_configuration: o
                .get_field("serverSideEncryptionConfiguration"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            updated_at: o.get_field("updatedAt"),
            user_context_policy: o.get_field("userContextPolicy"),
            user_group_resolution_configuration: o
                .get_field("userGroupResolutionConfiguration"),
            user_token_configurations: o.get_field("userTokenConfigurations"),
        }
    }
}
