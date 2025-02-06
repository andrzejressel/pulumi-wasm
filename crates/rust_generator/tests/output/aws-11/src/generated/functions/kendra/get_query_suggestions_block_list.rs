pub mod get_query_suggestions_block_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQuerySuggestionsBlockListArgs {
        /// Identifier of the index that contains the block list.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the block list.
        #[builder(into)]
        pub query_suggestions_block_list_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that helps organize the block list you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetQuerySuggestionsBlockListResult {
        /// ARN of the block list.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date-time a block list was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description for the block list.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Error message containing details if there are issues processing the block list.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// Current size of the block list text file in S3.
        pub file_size_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// Current number of valid, non-empty words or phrases in the block list text file.
        pub item_count: pulumi_gestalt_rust::Output<i32>,
        /// Name of the block list.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub query_suggestions_block_list_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of a role with permission to access the S3 bucket that contains the block list. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// S3 location of the block list input data. Detailed below.
        pub source_s3_paths: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::kendra::GetQuerySuggestionsBlockListSourceS3Path,
            >,
        >,
        /// Current status of the block list. When the value is `ACTIVE`, the block list is ready for use.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Metadata that helps organize the block list you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Date and time that the block list was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetQuerySuggestionsBlockListArgs,
    ) -> GetQuerySuggestionsBlockListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetQuerySuggestionsBlockListResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            error_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("errorMessage"),
            ),
            file_size_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileSizeBytes"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            index_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexId"),
            ),
            item_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("itemCount"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query_suggestions_block_list_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("querySuggestionsBlockListId"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            source_s3_paths: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceS3Paths"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            updated_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updatedAt"),
            ),
        }
    }
}
