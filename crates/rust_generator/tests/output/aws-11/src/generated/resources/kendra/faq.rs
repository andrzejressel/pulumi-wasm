/// Resource for managing an AWS Kendra FAQ.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kendra:Faq
///     properties:
///       indexId: ${exampleAwsKendraIndex.id}
///       name: Example
///       roleArn: ${exampleAwsIamRole.arn}
///       s3Path:
///         bucket: ${exampleAwsS3Bucket.id}
///         key: ${exampleAwsS3Object.key}
///       tags:
///         Name: Example Kendra Faq
/// ```
///
/// ### With File Format
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = faq::create(
///         "example",
///         FaqArgs::builder()
///             .file_format("CSV")
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("Example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .s_3_path(
///                 FaqS3Path::builder()
///                     .bucket("${exampleAwsS3Bucket.id}")
///                     .key("${exampleAwsS3Object.key}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Language Code
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = faq::create(
///         "example",
///         FaqArgs::builder()
///             .index_id("${exampleAwsKendraIndex.id}")
///             .language_code("en")
///             .name("Example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .s_3_path(
///                 FaqS3Path::builder()
///                     .bucket("${exampleAwsS3Bucket.id}")
///                     .key("${exampleAwsS3Object.key}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_kendra_faq` using the unique identifiers of the FAQ and index separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:kendra/faq:Faq example faq-123456780/idx-8012925589
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod faq {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FaqArgs {
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub file_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the index for a FAQ.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub language_code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name that should be associated with the FAQ.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the S3 bucket that contains the FAQs. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The S3 location of the FAQ input data. Detailed below.
        #[builder(into)]
        pub s3_path: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::kendra::FaqS3Path,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FaqResult {
        /// ARN of the FAQ.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Unix datetime that the FAQ was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// When the Status field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the FAQ.
        pub faq_id: pulumi_gestalt_rust::Output<String>,
        pub file_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier of the index for a FAQ.
        pub index_id: pulumi_gestalt_rust::Output<String>,
        pub language_code: pulumi_gestalt_rust::Output<String>,
        /// The name that should be associated with the FAQ.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the S3 bucket that contains the FAQs. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The S3 location of the FAQ input data. Detailed below.
        pub s3_path: pulumi_gestalt_rust::Output<super::super::types::kendra::FaqS3Path>,
        /// The status of the FAQ. It is ready to use when the status is ACTIVE.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The date and time that the FAQ was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FaqArgs,
    ) -> FaqResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let file_format_binding = args.file_format.get_output(context);
        let index_id_binding = args.index_id.get_output(context);
        let language_code_binding = args.language_code.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let s3_path_binding = args.s3_path.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kendra/faq:Faq".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileFormat".into(),
                    value: file_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexId".into(),
                    value: index_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageCode".into(),
                    value: language_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Path".into(),
                    value: s3_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FaqResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            error_message: o.get_field("errorMessage"),
            faq_id: o.get_field("faqId"),
            file_format: o.get_field("fileFormat"),
            index_id: o.get_field("indexId"),
            language_code: o.get_field("languageCode"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            s3_path: o.get_field("s3Path"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
