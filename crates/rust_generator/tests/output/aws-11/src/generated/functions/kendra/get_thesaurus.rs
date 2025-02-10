#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_thesaurus {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetThesaurusArgs {
        /// Identifier of the index that contains the Thesaurus.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that helps organize the Thesaurus you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the Thesaurus.
        #[builder(into)]
        pub thesaurus_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetThesaurusResult {
        /// ARN of the Thesaurus.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unix datetime that the Thesaurus was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the Thesaurus.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// When the `status` field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// Size of the Thesaurus file in bytes.
        pub file_size_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Thesaurus.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of a role with permission to access the S3 bucket that contains the Thesaurus. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// S3 location of the Thesaurus input data. Detailed below.
        pub source_s3_paths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetThesaurusSourceS3Path>,
        >,
        /// Status of the Thesaurus. It is ready to use when the status is `ACTIVE`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Number of synonym rules in the Thesaurus file.
        pub synonym_rule_count: pulumi_gestalt_rust::Output<i32>,
        /// Metadata that helps organize the Thesaurus you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Number of unique terms in the Thesaurus file. For example, the synonyms `a,b,c` and `a=>d`, the term count would be 4.
        pub term_count: pulumi_gestalt_rust::Output<i32>,
        pub thesaurus_id: pulumi_gestalt_rust::Output<String>,
        /// Date and time that the Thesaurus was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetThesaurusArgs,
    ) -> GetThesaurusResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let index_id_binding = args.index_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let thesaurus_id_binding = args.thesaurus_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kendra/getThesaurus:getThesaurus".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexId".into(),
                    value: index_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thesaurusId".into(),
                    value: thesaurus_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetThesaurusResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            error_message: o.get_field("errorMessage"),
            file_size_bytes: o.get_field("fileSizeBytes"),
            id: o.get_field("id"),
            index_id: o.get_field("indexId"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            source_s3_paths: o.get_field("sourceS3Paths"),
            status: o.get_field("status"),
            synonym_rule_count: o.get_field("synonymRuleCount"),
            tags: o.get_field("tags"),
            term_count: o.get_field("termCount"),
            thesaurus_id: o.get_field("thesaurusId"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
