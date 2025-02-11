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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QuerySuggestionsBlockListArgs,
    ) -> QuerySuggestionsBlockListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let index_id_binding = args.index_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let source_s3_path_binding = args.source_s3_path.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kendra/querySuggestionsBlockList:QuerySuggestionsBlockList"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceS3Path".into(),
                    value: &source_s3_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        QuerySuggestionsBlockListResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            index_id: o.get_field("indexId"),
            name: o.get_field("name"),
            query_suggestions_block_list_id: o.get_field("querySuggestionsBlockListId"),
            role_arn: o.get_field("roleArn"),
            source_s3_path: o.get_field("sourceS3Path"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
