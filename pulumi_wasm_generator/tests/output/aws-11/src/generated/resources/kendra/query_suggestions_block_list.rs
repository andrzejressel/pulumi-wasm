/// Use the `aws_kendra_index_block_list` resource to manage an AWS Kendra block list used for query suggestions for an index.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:QuerySuggestionsBlockList
///     properties:
///       indexId: ${exampleAwsKendraIndex.id}
///       name: Example
///       roleArn: ${exampleAwsIamRole.arn}
///       sourceS3Path:
///         bucket: ${exampleAwsS3Bucket.id}
///         key: example/suggestions.txt
///       tags:
///         Name: Example Kendra Index
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the `aws_kendra_query_suggestions_block_list` resource using the unique identifiers of the block list and index separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:kendra/querySuggestionsBlockList:QuerySuggestionsBlockList example blocklist-123456780/idx-8012925589
/// ```
pub mod query_suggestions_block_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QuerySuggestionsBlockListArgs {
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the index for a block list.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// Name for the block list.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// IAM (Identity and Access Management) role used to access the block list text file in S3.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// S3 path where your block list text file is located. See details below.
        #[builder(into)]
        pub source_s3_path: pulumi_wasm_rust::Output<
            super::super::types::kendra::QuerySuggestionsBlockListSourceS3Path,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QuerySuggestionsBlockListResult {
        /// ARN of the block list.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the index for a block list.
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// Name for the block list.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the block list.
        pub query_suggestions_block_list_id: pulumi_wasm_rust::Output<String>,
        /// IAM (Identity and Access Management) role used to access the block list text file in S3.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// S3 path where your block list text file is located. See details below.
        pub source_s3_path: pulumi_wasm_rust::Output<
            super::super::types::kendra::QuerySuggestionsBlockListSourceS3Path,
        >,
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider's default_tags configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: QuerySuggestionsBlockListArgs,
    ) -> QuerySuggestionsBlockListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let index_id_binding = args.index_id.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let source_s3_path_binding = args.source_s3_path.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kendra/querySuggestionsBlockList:QuerySuggestionsBlockList"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
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
                    name: "sourceS3Path".into(),
                    value: &source_s3_path_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "indexId".into(),
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
                    name: "sourceS3Path".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QuerySuggestionsBlockListResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            index_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexId").unwrap(),
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
            source_s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceS3Path").unwrap(),
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
        }
    }
}
