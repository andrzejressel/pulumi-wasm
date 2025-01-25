pub mod get_query_suggestions_block_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQuerySuggestionsBlockListArgs {
        /// Identifier of the index that contains the block list.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the block list.
        #[builder(into)]
        pub query_suggestions_block_list_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Metadata that helps organize the block list you create.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetQuerySuggestionsBlockListResult {
        /// ARN of the block list.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date-time a block list was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Description for the block list.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Error message containing details if there are issues processing the block list.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// Current size of the block list text file in S3.
        pub file_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// Current number of valid, non-empty words or phrases in the block list text file.
        pub item_count: pulumi_wasm_rust::Output<i32>,
        /// Name of the block list.
        pub name: pulumi_wasm_rust::Output<String>,
        pub query_suggestions_block_list_id: pulumi_wasm_rust::Output<String>,
        /// ARN of a role with permission to access the S3 bucket that contains the block list. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// S3 location of the block list input data. Detailed below.
        pub source_s3_paths: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::kendra::GetQuerySuggestionsBlockListSourceS3Path,
            >,
        >,
        /// Current status of the block list. When the value is `ACTIVE`, the block list is ready for use.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Metadata that helps organize the block list you create.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Date and time that the block list was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetQuerySuggestionsBlockListArgs,
    ) -> GetQuerySuggestionsBlockListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let index_id_binding = args.index_id.get_output(context).get_inner();
        let query_suggestions_block_list_id_binding = args
            .query_suggestions_block_list_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kendra/getQuerySuggestionsBlockList:getQuerySuggestionsBlockList"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
                },
                register_interface::ObjectField {
                    name: "querySuggestionsBlockListId".into(),
                    value: &query_suggestions_block_list_id_binding,
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
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "errorMessage".into(),
                },
                register_interface::ResultField {
                    name: "fileSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "indexId".into(),
                },
                register_interface::ResultField {
                    name: "itemCount".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "querySuggestionsBlockListId".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceS3Paths".into(),
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetQuerySuggestionsBlockListResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            error_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorMessage").unwrap(),
            ),
            file_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSizeBytes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            index_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexId").unwrap(),
            ),
            item_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("itemCount").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query_suggestions_block_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("querySuggestionsBlockListId").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            source_s3_paths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceS3Paths").unwrap(),
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
        }
    }
}
