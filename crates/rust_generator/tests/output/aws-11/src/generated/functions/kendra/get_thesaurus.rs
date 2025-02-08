#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetThesaurusArgs,
    ) -> GetThesaurusResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let index_id_binding = args.index_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let thesaurus_id_binding = args.thesaurus_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kendra/getThesaurus:getThesaurus".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "thesaurusId".into(),
                    value: &thesaurus_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetThesaurusResult {
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
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            source_s3_paths: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceS3Paths"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            synonym_rule_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("synonymRuleCount"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            term_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("termCount"),
            ),
            thesaurus_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thesaurusId"),
            ),
            updated_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updatedAt"),
            ),
        }
    }
}
