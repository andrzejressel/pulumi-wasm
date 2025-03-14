/// Provides a CloudWatch Log Data Protection Policy resource.
///
/// Read more about protecting sensitive user data in the [User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: example
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example
///   exampleLogDataProtectionPolicy:
///     type: aws:cloudwatch:LogDataProtectionPolicy
///     name: example
///     properties:
///       logGroupName: ${example.name}
///       policyDocument:
///         fn::toJSON:
///           Name: Example
///           Version: 2021-06-01
///           Statement:
///             - Sid: Audit
///               DataIdentifier:
///                 - arn:aws:dataprotection::aws:data-identifier/EmailAddress
///               Operation:
///                 Audit:
///                   FindingsDestination:
///                     S3:
///                       Bucket: ${exampleBucketV2.bucket}
///             - Sid: Redact
///               DataIdentifier:
///                 - arn:aws:dataprotection::aws:data-identifier/EmailAddress
///               Operation:
///                 Deidentify:
///                   MaskConfig: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import this resource using the `log_group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logDataProtectionPolicy:LogDataProtectionPolicy example my-log-group
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_data_protection_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogDataProtectionPolicyArgs {
        /// The name of the log group under which the log stream is to be created.
        #[builder(into)]
        pub log_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the data protection policy in JSON. Read more at [Data protection policy syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-start.html#mask-sensitive-log-data-policysyntax).
        #[builder(into)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogDataProtectionPolicyResult {
        /// The name of the log group under which the log stream is to be created.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the data protection policy in JSON. Read more at [Data protection policy syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/mask-sensitive-log-data-start.html#mask-sensitive-log-data-policysyntax).
        pub policy_document: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogDataProtectionPolicyArgs,
    ) -> LogDataProtectionPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_group_name_binding = args.log_group_name.get_output(context);
        let policy_document_binding = args.policy_document.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logDataProtectionPolicy:LogDataProtectionPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogDataProtectionPolicyResult {
            log_group_name: o.get_field("logGroupName"),
            policy_document: o.get_field("policyDocument"),
        }
    }
}
