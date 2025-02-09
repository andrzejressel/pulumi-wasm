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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod query_suggestions_block_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QuerySuggestionsBlockListArgs {
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the index for a block list.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name for the block list.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IAM (Identity and Access Management) role used to access the block list text file in S3.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// S3 path where your block list text file is located. See details below.
        #[builder(into)]
        pub source_s3_path: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::kendra::QuerySuggestionsBlockListSourceS3Path,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QuerySuggestionsBlockListResult {
        /// ARN of the block list.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the index for a block list.
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// Name for the block list.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the block list.
        pub query_suggestions_block_list_id: pulumi_gestalt_rust::Output<String>,
        /// IAM (Identity and Access Management) role used to access the block list text file in S3.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// S3 path where your block list text file is located. See details below.
        pub source_s3_path: pulumi_gestalt_rust::Output<
            super::super::types::kendra::QuerySuggestionsBlockListSourceS3Path,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider's default_tags configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: QuerySuggestionsBlockListArgs,
    ) -> QuerySuggestionsBlockListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let index_id_binding_1 = args.index_id.get_output(context);
        let index_id_binding = index_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let source_s3_path_binding_1 = args.source_s3_path.get_output(context);
        let source_s3_path_binding = source_s3_path_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        QuerySuggestionsBlockListResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            index_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query_suggestions_block_list_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("querySuggestionsBlockListId"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            source_s3_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceS3Path"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
