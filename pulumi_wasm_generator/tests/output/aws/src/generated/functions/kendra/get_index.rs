pub mod get_index {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIndexArgs {
        /// Returns information on a specific Index by id.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
        /// Metadata that helps organize the Indices you create.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetIndexResult {
        /// ARN of the Index.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Block that sets the number of additional document storage and query capacity units that should be used by the index. Documented below.
        pub capacity_units: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kendra::GetIndexCapacityUnit>,
        >,
        /// Unix datetime that the index was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Description of the Index.
        pub description: pulumi_wasm_rust::Output<String>,
        /// One or more blocks that specify the configuration settings for any metadata applied to the documents in the index. Documented below.
        pub document_metadata_configuration_updates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::kendra::GetIndexDocumentMetadataConfigurationUpdate,
            >,
        >,
        /// Amazon Kendra edition for the index.
        pub edition: pulumi_wasm_rust::Output<String>,
        /// When the Status field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// Identifier of the Index.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Block that provides information about the number of FAQ questions and answers and the number of text documents indexed. Documented below.
        pub index_statistics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kendra::GetIndexIndexStatistic>,
        >,
        /// Name of the index field. Minimum length of 1. Maximum length of 30.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access your Amazon CloudWatch logs and metrics. This is also the role you use when you call the `BatchPutDocument` API to index documents from an Amazon S3 bucket.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A block that specifies the identifier of the AWS KMS customer managed key (CMK) that's used to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs. Documented below.
        pub server_side_encryption_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::kendra::GetIndexServerSideEncryptionConfiguration,
            >,
        >,
        /// Current status of the index. When the value is `ACTIVE`, the index is ready for use. If the Status field value is `FAILED`, the `error_message` field contains a message that explains why.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Metadata that helps organize the Indices you create.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Unix datetime that the index was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
        /// User context policy. Valid values are `ATTRIBUTE_FILTER` or `USER_TOKEN`. For more information, refer to [UserContextPolicy](https://docs.aws.amazon.com/kendra/latest/APIReference/API_CreateIndex.html#kendra-CreateIndex-request-UserContextPolicy).
        pub user_context_policy: pulumi_wasm_rust::Output<String>,
        /// A block that enables fetching access levels of groups and users from an AWS Single Sign-On identity source. Documented below.
        pub user_group_resolution_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::kendra::GetIndexUserGroupResolutionConfiguration,
            >,
        >,
        /// A block that specifies the user token configuration. Documented below.
        pub user_token_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kendra::GetIndexUserTokenConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetIndexArgs) -> GetIndexResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kendra/getIndex:getIndex".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
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
                    name: "capacityUnits".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "documentMetadataConfigurationUpdates".into(),
                },
                register_interface::ResultField {
                    name: "edition".into(),
                },
                register_interface::ResultField {
                    name: "errorMessage".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "indexStatistics".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
                register_interface::ResultField {
                    name: "userContextPolicy".into(),
                },
                register_interface::ResultField {
                    name: "userGroupResolutionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "userTokenConfigurations".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetIndexResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            capacity_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityUnits").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            document_metadata_configuration_updates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentMetadataConfigurationUpdates").unwrap(),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edition").unwrap(),
            ),
            error_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorMessage").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            index_statistics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexStatistics").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            server_side_encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryptionConfigurations").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
            user_context_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userContextPolicy").unwrap(),
            ),
            user_group_resolution_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userGroupResolutionConfigurations").unwrap(),
            ),
            user_token_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userTokenConfigurations").unwrap(),
            ),
        }
    }
}
