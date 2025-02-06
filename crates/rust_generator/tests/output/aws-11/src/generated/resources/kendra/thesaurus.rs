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
pub mod thesaurus {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThesaurusArgs {
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The identifier of the index for a thesaurus.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name for the thesaurus.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IAM (Identity and Access Management) role used to access the thesaurus file in S3.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The S3 path where your thesaurus file sits in S3. Detailed below.
        #[builder(into)]
        pub source_s3_path: pulumi_wasm_rust::InputOrOutput<
            super::super::types::kendra::ThesaurusSourceS3Path,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ThesaurusResult {
        /// ARN of the thesaurus.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the index for a thesaurus.
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// The name for the thesaurus.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The IAM (Identity and Access Management) role used to access the thesaurus file in S3.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The S3 path where your thesaurus file sits in S3. Detailed below.
        pub source_s3_path: pulumi_wasm_rust::Output<
            super::super::types::kendra::ThesaurusSourceS3Path,
        >,
        /// The current status of the thesaurus.
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub thesaurus_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ThesaurusArgs,
    ) -> ThesaurusResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let index_id_binding = args.index_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let source_s3_path_binding = args.source_s3_path.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kendra/thesaurus:Thesaurus".into(),
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
        ThesaurusResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            index_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            source_s3_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceS3Path"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            thesaurus_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thesaurusId"),
            ),
        }
    }
}
