/// Resource for managing an AWS Kendra Thesaurus.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:Thesaurus
///     properties:
///       indexId: ${exampleAwsKendraIndex.id}
///       name: Example
///       roleArn: ${exampleAwsIamRole.arn}
///       sourceS3Path:
///         bucket: ${exampleAwsS3Bucket.id}
///         key: ${exampleAwsS3Object.key}
///       tags:
///         Name: Example Kendra Thesaurus
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_kendra_thesaurus` using the unique identifiers of the thesaurus and index separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:kendra/thesaurus:Thesaurus example thesaurus-123456780/idx-8012925589
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod thesaurus {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThesaurusArgs {
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the index for a thesaurus.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for the thesaurus.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM (Identity and Access Management) role used to access the thesaurus file in S3.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The S3 path where your thesaurus file sits in S3. Detailed below.
        #[builder(into)]
        pub source_s3_path: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::kendra::ThesaurusSourceS3Path,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ThesaurusResult {
        /// ARN of the thesaurus.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier of the index for a thesaurus.
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// The name for the thesaurus.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The IAM (Identity and Access Management) role used to access the thesaurus file in S3.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The S3 path where your thesaurus file sits in S3. Detailed below.
        pub source_s3_path: pulumi_gestalt_rust::Output<
            super::super::types::kendra::ThesaurusSourceS3Path,
        >,
        /// The current status of the thesaurus.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub thesaurus_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThesaurusArgs,
    ) -> ThesaurusResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let index_id_binding = args.index_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let source_s3_path_binding = args.source_s3_path.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kendra/thesaurus:Thesaurus".into(),
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
        ThesaurusResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            index_id: o.get_field("indexId"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            source_s3_path: o.get_field("sourceS3Path"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            thesaurus_id: o.get_field("thesaurusId"),
        }
    }
}
