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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod faq {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FaqArgs {
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub file_format: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the index for a FAQ.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub language_code: pulumi_wasm_rust::Output<Option<String>>,
        /// The name that should be associated with the FAQ.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the S3 bucket that contains the FAQs. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The S3 location of the FAQ input data. Detailed below.
        #[builder(into)]
        pub s3_path: pulumi_wasm_rust::Output<super::super::types::kendra::FaqS3Path>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FaqResult {
        /// ARN of the FAQ.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Unix datetime that the FAQ was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// When the Status field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// The identifier of the FAQ.
        pub faq_id: pulumi_wasm_rust::Output<String>,
        pub file_format: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the index for a FAQ.
        pub index_id: pulumi_wasm_rust::Output<String>,
        pub language_code: pulumi_wasm_rust::Output<String>,
        /// The name that should be associated with the FAQ.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of a role with permission to access the S3 bucket that contains the FAQs. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The S3 location of the FAQ input data. Detailed below.
        pub s3_path: pulumi_wasm_rust::Output<super::super::types::kendra::FaqS3Path>,
        /// The status of the FAQ. It is ready to use when the status is ACTIVE.
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The date and time that the FAQ was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FaqArgs) -> FaqResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let file_format_binding = args.file_format.get_inner();
        let index_id_binding = args.index_id.get_inner();
        let language_code_binding = args.language_code.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let s3_path_binding = args.s3_path.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kendra/faq:Faq".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "fileFormat".into(),
                    value: &file_format_binding,
                },
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
                },
                register_interface::ObjectField {
                    name: "languageCode".into(),
                    value: &language_code_binding,
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
                    name: "s3Path".into(),
                    value: &s3_path_binding,
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
                    name: "faqId".into(),
                },
                register_interface::ResultField {
                    name: "fileFormat".into(),
                },
                register_interface::ResultField {
                    name: "indexId".into(),
                },
                register_interface::ResultField {
                    name: "languageCode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "s3Path".into(),
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
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FaqResult {
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
            faq_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("faqId").unwrap(),
            ),
            file_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileFormat").unwrap(),
            ),
            index_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexId").unwrap(),
            ),
            language_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageCode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            s3_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3Path").unwrap(),
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
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}