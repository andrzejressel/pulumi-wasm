/// Resource for managing an Amazon Customer Profiles Domain.
/// See the [Create Domain](https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_CreateDomain.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder().domain_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ### With SQS DLQ and KMS set
///
/// ```yaml
/// resources:
///   example:
///     type: aws:sqs:Queue
///     properties:
///       name: example
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: Customer Profiles SQS policy
///               Effect: Allow
///               Action:
///                 - sqs:SendMessage
///               Resource: '*'
///               Principal:
///                 Service: profile.amazonaws.com
///   exampleKey:
///     type: aws:kms:Key
///     name: example
///     properties:
///       description: example
///       deletionWindowInDays: 10
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example
///       forceDestroy: true
///   exampleBucketPolicy:
///     type: aws:s3:BucketPolicy
///     name: example
///     properties:
///       bucket: ${exampleBucketV2.id}
///       policy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: Customer Profiles S3 policy
///               Effect: Allow
///               Action:
///                 - s3:GetObject
///                 - s3:PutObject
///                 - s3:ListBucket
///               Resource:
///                 - ${exampleBucketV2.arn}
///                 - ${exampleBucketV2.arn}/*
///               Principal:
///                 Service: profile.amazonaws.com
///   test:
///     type: aws:customerprofiles:Domain
///     properties:
///       domainName: ${example}
///       deadLetterQueueUrl: ${example.id}
///       defaultEncryptionKey: ${exampleKey.arn}
///       defaultExpirationDays: 365
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Customer Profiles Domain using the resource `id`. For example:
///
/// ```sh
/// $ pulumi import aws:customerprofiles/domain:Domain example e6f777be-22d0-4b40-b307-5d2720ef16b2
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The URL of the SQS dead letter queue, which is used for reporting errors associated with ingesting data from third party applications.
        #[builder(into, default)]
        pub dead_letter_queue_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The default encryption key, which is an AWS managed key, is used when no specific type of encryption key is specified. It is used to encrypt all data before it is placed in permanent or semi-permanent storage.
        #[builder(into, default)]
        pub default_encryption_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The default number of days until the data within the domain expires.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub default_expiration_days: pulumi_wasm_rust::Output<i32>,
        /// The name for your Customer Profile domain. It must be unique for your AWS account.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// A block that specifies the process of matching duplicate profiles. Documented below.
        #[builder(into, default)]
        pub matching: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::DomainMatching>,
        >,
        /// A block that specifies the process of matching duplicate profiles using the Rule-Based matching. Documented below.
        #[builder(into, default)]
        pub rule_based_matching: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::DomainRuleBasedMatching>,
        >,
        /// Tags to apply to the domain. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The Amazon Resource Name (ARN) of the Customer Profiles Domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The URL of the SQS dead letter queue, which is used for reporting errors associated with ingesting data from third party applications.
        pub dead_letter_queue_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The default encryption key, which is an AWS managed key, is used when no specific type of encryption key is specified. It is used to encrypt all data before it is placed in permanent or semi-permanent storage.
        pub default_encryption_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The default number of days until the data within the domain expires.
        ///
        /// The following arguments are optional:
        pub default_expiration_days: pulumi_wasm_rust::Output<i32>,
        /// The name for your Customer Profile domain. It must be unique for your AWS account.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// A block that specifies the process of matching duplicate profiles. Documented below.
        pub matching: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::DomainMatching>,
        >,
        /// A block that specifies the process of matching duplicate profiles using the Rule-Based matching. Documented below.
        pub rule_based_matching: pulumi_wasm_rust::Output<
            Option<super::super::types::customerprofiles::DomainRuleBasedMatching>,
        >,
        /// Tags to apply to the domain. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dead_letter_queue_url_binding = args.dead_letter_queue_url.get_inner();
        let default_encryption_key_binding = args.default_encryption_key.get_inner();
        let default_expiration_days_binding = args.default_expiration_days.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let matching_binding = args.matching.get_inner();
        let rule_based_matching_binding = args.rule_based_matching.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:customerprofiles/domain:Domain".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deadLetterQueueUrl".into(),
                    value: &dead_letter_queue_url_binding,
                },
                register_interface::ObjectField {
                    name: "defaultEncryptionKey".into(),
                    value: &default_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "defaultExpirationDays".into(),
                    value: &default_expiration_days_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "matching".into(),
                    value: &matching_binding,
                },
                register_interface::ObjectField {
                    name: "ruleBasedMatching".into(),
                    value: &rule_based_matching_binding,
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
                    name: "deadLetterQueueUrl".into(),
                },
                register_interface::ResultField {
                    name: "defaultEncryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultExpirationDays".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "matching".into(),
                },
                register_interface::ResultField {
                    name: "ruleBasedMatching".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            dead_letter_queue_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deadLetterQueueUrl").unwrap(),
            ),
            default_encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultEncryptionKey").unwrap(),
            ),
            default_expiration_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultExpirationDays").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            matching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("matching").unwrap(),
            ),
            rule_based_matching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleBasedMatching").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}