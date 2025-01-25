pub mod get_thesaurus {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetThesaurusArgs {
        /// Identifier of the index that contains the Thesaurus.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Metadata that helps organize the Thesaurus you create.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the Thesaurus.
        #[builder(into)]
        pub thesaurus_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetThesaurusResult {
        /// ARN of the Thesaurus.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Unix datetime that the Thesaurus was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Description of the Thesaurus.
        pub description: pulumi_wasm_rust::Output<String>,
        /// When the `status` field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// Size of the Thesaurus file in bytes.
        pub file_size_bytes: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Thesaurus.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of a role with permission to access the S3 bucket that contains the Thesaurus. For more information, see [IAM Roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// S3 location of the Thesaurus input data. Detailed below.
        pub source_s3_paths: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kendra::GetThesaurusSourceS3Path>,
        >,
        /// Status of the Thesaurus. It is ready to use when the status is `ACTIVE`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Number of synonym rules in the Thesaurus file.
        pub synonym_rule_count: pulumi_wasm_rust::Output<i32>,
        /// Metadata that helps organize the Thesaurus you create.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Number of unique terms in the Thesaurus file. For example, the synonyms `a,b,c` and `a=>d`, the term count would be 4.
        pub term_count: pulumi_wasm_rust::Output<i32>,
        pub thesaurus_id: pulumi_wasm_rust::Output<String>,
        /// Date and time that the Thesaurus was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetThesaurusArgs,
    ) -> GetThesaurusResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
                    name: "fileSizeBytes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "indexId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceS3Paths".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "synonymRuleCount".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "termCount".into(),
                },
                register_interface::ResultField {
                    name: "thesaurusId".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetThesaurusResult {
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
            file_size_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSizeBytes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            index_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            source_s3_paths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceS3Paths").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            synonym_rule_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synonymRuleCount").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            term_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("termCount").unwrap(),
            ),
            thesaurus_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thesaurusId").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}
