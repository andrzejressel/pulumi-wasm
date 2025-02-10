/// Resource for managing an Amazon Customer Profiles Domain.
/// See the [Create Domain](https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_CreateDomain.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The URL of the SQS dead letter queue, which is used for reporting errors associated with ingesting data from third party applications.
        #[builder(into, default)]
        pub dead_letter_queue_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default encryption key, which is an AWS managed key, is used when no specific type of encryption key is specified. It is used to encrypt all data before it is placed in permanent or semi-permanent storage.
        #[builder(into, default)]
        pub default_encryption_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default number of days until the data within the domain expires.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub default_expiration_days: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The name for your Customer Profile domain. It must be unique for your AWS account.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A block that specifies the process of matching duplicate profiles. Documented below.
        #[builder(into, default)]
        pub matching: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::customerprofiles::DomainMatching>,
        >,
        /// A block that specifies the process of matching duplicate profiles using the Rule-Based matching. Documented below.
        #[builder(into, default)]
        pub rule_based_matching: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::customerprofiles::DomainRuleBasedMatching>,
        >,
        /// Tags to apply to the domain. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The Amazon Resource Name (ARN) of the Customer Profiles Domain.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The URL of the SQS dead letter queue, which is used for reporting errors associated with ingesting data from third party applications.
        pub dead_letter_queue_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default encryption key, which is an AWS managed key, is used when no specific type of encryption key is specified. It is used to encrypt all data before it is placed in permanent or semi-permanent storage.
        pub default_encryption_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default number of days until the data within the domain expires.
        ///
        /// The following arguments are optional:
        pub default_expiration_days: pulumi_gestalt_rust::Output<i32>,
        /// The name for your Customer Profile domain. It must be unique for your AWS account.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// A block that specifies the process of matching duplicate profiles. Documented below.
        pub matching: pulumi_gestalt_rust::Output<
            Option<super::super::types::customerprofiles::DomainMatching>,
        >,
        /// A block that specifies the process of matching duplicate profiles using the Rule-Based matching. Documented below.
        pub rule_based_matching: pulumi_gestalt_rust::Output<
            Option<super::super::types::customerprofiles::DomainRuleBasedMatching>,
        >,
        /// Tags to apply to the domain. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dead_letter_queue_url_binding = args
            .dead_letter_queue_url
            .get_output(context);
        let default_encryption_key_binding = args
            .default_encryption_key
            .get_output(context);
        let default_expiration_days_binding = args
            .default_expiration_days
            .get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let matching_binding = args.matching.get_output(context);
        let rule_based_matching_binding = args.rule_based_matching.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:customerprofiles/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterQueueUrl".into(),
                    value: dead_letter_queue_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultEncryptionKey".into(),
                    value: default_encryption_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultExpirationDays".into(),
                    value: default_expiration_days_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matching".into(),
                    value: matching_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleBasedMatching".into(),
                    value: rule_based_matching_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainResult {
            arn: o.get_field("arn"),
            dead_letter_queue_url: o.get_field("deadLetterQueueUrl"),
            default_encryption_key: o.get_field("defaultEncryptionKey"),
            default_expiration_days: o.get_field("defaultExpirationDays"),
            domain_name: o.get_field("domainName"),
            matching: o.get_field("matching"),
            rule_based_matching: o.get_field("ruleBasedMatching"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
