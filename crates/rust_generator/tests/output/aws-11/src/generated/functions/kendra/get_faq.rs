#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_faq {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFaqArgs {
        /// Identifier of the FAQ.
        #[builder(into)]
        pub faq_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the index that contains the FAQ.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that helps organize the FAQs you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFaqResult {
        /// ARN of the FAQ.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unix datetime that the faq was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the FAQ.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// When the `status` field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        pub faq_id: pulumi_gestalt_rust::Output<String>,
        /// File format used by the input files for the FAQ. Valid Values are `CSV`, `CSV_WITH_HEADER`, `JSON`.
        pub file_format: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// Code for a language. This shows a supported language for the FAQ document. For more information on supported languages, including their codes, see [Adding documents in languages other than English](https://docs.aws.amazon.com/kendra/latest/dg/in-adding-languages.html).
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// Name of the FAQ.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of a role with permission to access the S3 bucket that contains the FAQs. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// S3 location of the FAQ input data. Detailed below.
        pub s3_paths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetFaqS3Path>,
        >,
        /// Status of the FAQ. It is ready to use when the status is ACTIVE.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Metadata that helps organize the FAQs you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Date and time that the FAQ was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFaqArgs,
    ) -> GetFaqResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let faq_id_binding = args.faq_id.get_output(context).get_inner();
        let index_id_binding = args.index_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kendra/getFaq:getFaq".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "faqId".into(),
                    value: &faq_id_binding,
                },
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFaqResult {
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
            faq_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("faqId"),
            ),
            file_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fileFormat"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            index_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexId"),
            ),
            language_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("languageCode"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            s3_paths: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3Paths"),
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
